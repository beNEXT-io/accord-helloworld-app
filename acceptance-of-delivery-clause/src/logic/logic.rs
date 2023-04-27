use std::{cmp::Ordering, collections::HashMap, process::Command, str::FromStr};

use lib::{
    org_accordproject_acceptanceofdelivery_0_15_0::{
        AcceptanceOfDeliveryClause, InspectDeliverable, InspectionResponse, InspectionStatus,
    },
    org_accordproject_organization_0_2_0::Organization,
    org_accordproject_runtime_0_2_0::*,
    utils::*,
};

use chrono::offset::Utc;
use chrono::{naive::Days, DateTime};
use lambda_http::{Body, Error, IntoResponse, Request, Response};
use serde_json::{from_str, from_value, Value};

const CONTRACT_STR: &str = r#"
    {
        "$class": "org.accordproject.acceptanceofdelivery.AcceptanceOfDeliveryClause",
        "shipper": "resource:org.accordproject.organization.Organization#Party%20A",
        "receiver": "resource:org.accordproject.organization.Organization#Party%20B",
        "deliverable": "Widgets",
        "businessDays": 10,
        "attachment": "Attachment X",
        "clauseId": "16ddc768-3847-4fc6-a7e4-9b7d94c020dd",
        "$identifier": "16ddc768-3847-4fc6-a7e4-9b7d94c020dd"
    }
"#;

const DELIVERABLE: &str = r#"
    {
        "$class": "org.accordproject.acceptanceofdelivery.InspectDeliverable",
        "deliverableReceivedAt": "2019-01-22 03:24:00Z",
        "inspectionPassed": true
    }
"#;

fn trim(s: &Value) -> String {
    let Some(s) = s.as_str() else {
                panic!("cannot convert JSON field value to string")
            };

    s.trim_matches('\"').to_string()
}

pub struct AcceptanceOfDeliveryClauseImpl(AcceptanceOfDeliveryClause);
pub struct OrganizationImpl(Organization);
pub struct InspectDeliverableImpl(InspectDeliverable);
pub struct InspectionResponseImpl(InspectionResponse);

impl OrganizationImpl {
    pub fn new(field_name: &'static str, field_value: String) -> Self {
        let organization = r#"
            {
                "$class": "org.accordproject.organization.Organization",
                "identifier": "resource1",
                "$identifier": "resource1"
            }
        "#;
        let organization: HashMap<&str, &str> = from_str(&organization).unwrap();

        let Some(identifier) = field_value
            .split("#")
            .last()
            .and_then(|s| {
                Some(s.replace("%20", " ").trim_matches('\"').to_string())
            }) else {
                panic!("Cannot find the field value")
            };

        Self(Organization {
            _class: organization[&"$class"].to_string(),
            identifier: identifier.clone(),
            name: Some(field_name.to_string()),
            description: None,
            duns: None,
            place: None,
            _identifier: identifier,
        })
    }
}

impl AcceptanceOfDeliveryClauseImpl {
    pub fn new(contract: &'static str) -> Self {
        let Ok(contract_map) = serde_json::from_str::<HashMap<&'static str, Value>>(&contract) else {
            panic!("Cannot convert Acceptance of Delivery Clause to HashMap")
        };

        Self(AcceptanceOfDeliveryClause {
            _class: trim(&contract_map[&"$class"]),
            shipper: OrganizationImpl::new("shipper", contract_map[&"shipper"].to_string()).0,
            receiver: OrganizationImpl::new("receiver", contract_map[&"receiver"].to_string()).0,
            deliverable: trim(&contract_map[&"deliverable"]),
            business_days: contract_map[&"businessDays"]
                .as_u64()
                .unwrap_or_else(|| panic!("cannot convert business days to u64")),
            attachment: trim(&contract_map[&"attachment"]),
            clause_id: trim(&contract_map[&"clauseId"]),
            _identifier: trim(&contract_map[&"$identifier"]),
        })
    }
}

impl<'a> InspectDeliverableImpl {
    pub fn new(
        _class: String,
        deliverable_received_at: DateTime<Utc>,
        inspection_passed: bool,
        _timestamp: DateTime<Utc>,
    ) -> Self {
        Self(InspectDeliverable {
            _class,
            deliverable_received_at,
            inspection_passed,
            _timestamp,
        })
    }
    pub fn from_request(request: Request) -> Result<Self, &'static str> {
        match request.body().clone() {
            Body::Empty => Err("The request is empty"),
            Body::Text(s) => Ok(Self::from_str(s.clone().as_str())),
            Body::Binary(_) => Err("The request contains binary data which is not handled"),
        }
    }

    pub fn from_str(s: &'a str) -> Self {
        let deliverable = from_str::<HashMap<&'a str, Value>>(s)
            .unwrap_or_else(|_| panic!("Cannot convert to InspectDeliverable"));

        Self::new(
            trim(&deliverable[&"$class"]),
            from_value(deliverable[&"deliverableReceivedAt"].clone())
                .unwrap_or_else(|_| panic!("Cannot deserialize datetime")),
            deliverable[&"inspectionPassed"]
                .as_bool()
                .unwrap_or_else(|| panic!("cannot get inspection_passed from the json")),
            Utc::now(),
        )
    }
}

impl<'a> InspectionResponseImpl {
    pub fn new(
        status: InspectionStatus,
        shipper: Organization,
        receiver: Organization,
        timestamp: DateTime<Utc>,
    ) -> InspectionResponseImpl {
        Self(InspectionResponse {
            _class: "org.accordproject.acceptanceofdelivery.InspectionResponse".into(),
            status,
            shipper,
            receiver,
            _timestamp: timestamp, // this is not ideal
        })
    }

    pub fn into_response(self) -> Result<Response<Body>, &'a str> {
        let response = serde_json::to_string(&self.0)
            .map_err(|_| "Unable convert InspectionResponse struct to string")?;

        Response::builder()
            .status(200)
            .header("content-type", "application/json")
            .body(response.into())
            .map_err(|_| "Unable to build response using the InspectionResponse string")
    }
}

fn now(s: &'static str) -> DateTime<Utc> {
    DateTime::from_str(s).unwrap_or_else(|_| panic!("Cannot convert string to UTC datetime format"))
}

pub async fn handler(request: Request) -> Result<Response<Body>, &'static str> {
    let now = now("2019-01-31T16:34:00-05:00");
    let contract = AcceptanceOfDeliveryClauseImpl::new(CONTRACT_STR).0;
    let request = InspectDeliverableImpl::from_request(request)?.0;
    let received = request.deliverable_received_at;
    if now < received {
        return Err("Transaction time is before the deliverable date.");
    };

    let status: InspectionStatus = {
        match received.checked_add_days(Days::new(contract.business_days)) {
            None => Err("This is not a valid date"),
            Some(date) => Ok({
                if now < date {
                    if request.inspection_passed {
                        InspectionStatus::PASSED_TESTING
                    } else {
                        InspectionStatus::FAILED_TESTING
                    }
                } else {
                    InspectionStatus::OUTSIDE_INSPECTION_PERIOD
                }
            }),
        }
    }?;

    let response = InspectionResponseImpl::new(status, contract.shipper, contract.receiver, now);

    response.into_response()
}
