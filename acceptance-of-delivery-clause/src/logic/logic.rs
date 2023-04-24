use std::cmp::Ordering;

use lib::{
    org_accordproject_acceptanceofdelivery_0_15_0::{
        AcceptanceOfDeliveryClause, InspectDeliverable, InspectionResponse, InspectionStatus,
    },
    org_accordproject_runtime_0_2_0::*,
};

use chrono::offset::Utc;
use chrono::{naive::Days, DateTime};

pub fn acceptance_of_delivery(
    contract: AcceptanceOfDeliveryClause,
    request: InspectDeliverable,
) -> Result<InspectionResponse, &'static str> {
    let received = request.deliverable_received_at;
    if received <= Utc::now() {
        return Err("Transaction time is before the deliverable date.");
    }

    let status: InspectionStatus = {
        match received.checked_add_days(Days::new(contract.business_days)) {
            None => {
                if request.inspection_passed {
                    InspectionStatus::PASSED_TESTING
                } else {
                    InspectionStatus::FAILED_TESTING
                }
            }
            Some(_) => InspectionStatus::OUTSIDE_INSPECTION_PERIOD,
        }
    };

    return Ok(InspectionResponse {
        _class: String::new(), // this is not ideal
        status,
        shipper: contract.shipper,
        receiver: contract.receiver,
        _timestamp: Utc::now(), // this is not ideal
    });
}
