use serde::{ Deserialize, Serialize };
use chrono::{ DateTime, TimeZone, Utc };
   
use crate::lib::org_accordproject_address::*;
use crate::lib::concerto_1_0_0::*;
use crate::lib::utils::*;
   
#[derive(Debug, Serialize, Deserialize)]
pub struct GeoCoordinates {
   #[serde(
      rename = "$class",
   )]
   pub _class: String,
   
   #[serde(
      rename = "address",
      skip_serializing_if = "Option::is_none",
   )]
   pub address: Option<PostalAddress>,
   
   #[serde(
      rename = "addressCountry",
      skip_serializing_if = "Option::is_none",
   )]
   pub address_country: Option<String>,
   
   #[serde(
      rename = "elevation",
      skip_serializing_if = "Option::is_none",
   )]
   pub elevation: Option<f64>,
   
   #[serde(
      rename = "latitude",
      skip_serializing_if = "Option::is_none",
   )]
   pub latitude: Option<f64>,
   
   #[serde(
      rename = "longitude",
      skip_serializing_if = "Option::is_none",
   )]
   pub longitude: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Place {
   #[serde(
      rename = "$class",
   )]
   pub _class: String,
   
   #[serde(
      rename = "address",
      skip_serializing_if = "Option::is_none",
   )]
   pub address: Option<PostalAddress>,
   
   #[serde(
      rename = "branchCode",
      skip_serializing_if = "Option::is_none",
   )]
   pub branch_code: Option<String>,
   
   #[serde(
      rename = "faxNumber",
      skip_serializing_if = "Option::is_none",
   )]
   pub fax_number: Option<String>,
   
   #[serde(
      rename = "geo",
      skip_serializing_if = "Option::is_none",
   )]
   pub geo: Option<GeoCoordinates>,
   
   #[serde(
      rename = "globalLocationNumber",
      skip_serializing_if = "Option::is_none",
   )]
   pub global_location_number: Option<String>,
   
   #[serde(
      rename = "mapUrl",
      skip_serializing_if = "Option::is_none",
   )]
   pub map_url: Option<String>,
   
   #[serde(
      rename = "isicsV4",
      skip_serializing_if = "Option::is_none",
   )]
   pub isics_v4: Option<String>,
   
   #[serde(
      rename = "telephone",
      skip_serializing_if = "Option::is_none",
   )]
   pub telephone: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Country {
   #[serde(
      rename = "$class",
   )]
   pub _class: String,
   
   #[serde(
      rename = "name",
   )]
   pub name: String,
   
   #[serde(
      rename = "optional",
   )]
   pub optional: CountryCodeISOAlpha2,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum CountryCodeISOAlpha2 {
   AD,
   AE,
   AF,
   AG,
   AI,
   AL,
   AM,
   AO,
   AQ,
   AR,
   AS,
   AT,
   AU,
   AW,
   AX,
   AZ,
   BA,
   BB,
   BD,
   BE,
   BF,
   BG,
   BH,
   BI,
   BJ,
   BL,
   BM,
   BN,
   BO,
   BQ,
   BR,
   BS,
   BT,
   BV,
   BW,
   BY,
   BZ,
   CA,
   CC,
   CD,
   CF,
   CG,
   CH,
   CI,
   CK,
   CL,
   CM,
   CN,
   CO,
   CR,
   CU,
   CV,
   CW,
   CX,
   CY,
   CZ,
   DE,
   DJ,
   DK,
   DM,
   DO,
   DZ,
   EC,
   EE,
   EG,
   EH,
   ER,
   ES,
   ET,
   FI,
   FJ,
   FK,
   FM,
   FO,
   FR,
   GA,
   GB,
   GD,
   GE,
   GF,
   GG,
   GH,
   GI,
   GL,
   GM,
   GN,
   GP,
   GQ,
   GR,
   GS,
   GT,
   GU,
   GW,
   GY,
   HK,
   HM,
   HN,
   HR,
   HT,
   HU,
   ID,
   IE,
   IL,
   IM,
   IN,
   IO,
   IQ,
   IR,
   IS,
   IT,
   JE,
   JM,
   JO,
   JP,
   KE,
   KG,
   KH,
   KI,
   KM,
   KN,
   KP,
   KR,
   KW,
   KY,
   KZ,
   LA,
   LB,
   LC,
   LI,
   LK,
   LR,
   LS,
   LT,
   LU,
   LV,
   LY,
   MA,
   MC,
   MD,
   ME,
   MF,
   MG,
   MH,
   MK,
   ML,
   MM,
   MN,
   MO,
   MP,
   MQ,
   MR,
   MS,
   MT,
   MU,
   MV,
   MW,
   MX,
   MY,
   MZ,
   NA,
   NC,
   NE,
   NF,
   NG,
   NI,
   NL,
   NO,
   NP,
   NR,
   NU,
   NZ,
   OM,
   PA,
   PE,
   PF,
   PG,
   PH,
   PK,
   PL,
   PM,
   PN,
   PR,
   PS,
   PT,
   PW,
   PY,
   QA,
   RE,
   RO,
   RS,
   RU,
   RW,
   SA,
   SB,
   SC,
   SD,
   SE,
   SG,
   SH,
   SI,
   SJ,
   SK,
   SL,
   SM,
   SN,
   SO,
   SR,
   SS,
   ST,
   SV,
   SX,
   SY,
   SZ,
   TC,
   TD,
   TF,
   TG,
   TH,
   TJ,
   TK,
   TL,
   TM,
   TN,
   TO,
   TR,
   TT,
   TV,
   TW,
   TZ,
   UA,
   UG,
   UM,
   US,
   UY,
   UZ,
   VA,
   VC,
   VE,
   VG,
   VI,
   VN,
   VU,
   WF,
   WS,
   YE,
   YT,
   ZA,
   ZM,
   ZW,
}

