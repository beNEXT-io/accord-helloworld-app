use serde::{ Deserialize, Serialize };
use chrono::{ DateTime, TimeZone, Utc };
   
use crate::concerto_1_0_0::*;
use crate::utils::*;
   
#[derive(Debug, Serialize, Deserialize)]
pub struct DigitalMonetaryAmount {
   #[serde(
      rename = "$class",
   )]
   pub _class: String,
   
   #[serde(
      rename = "doubleValue",
   )]
   pub double_value: f64,
   
   #[serde(
      rename = "digitalCurrencyCode",
   )]
   pub digital_currency_code: DigitalCurrencyCode,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum DigitalCurrencyCode {
   #[allow(non_camel_case_types)]
   ADA,
   #[allow(non_camel_case_types)]
   BCH,
   #[allow(non_camel_case_types)]
   BTC,
   #[allow(non_camel_case_types)]
   DASH,
   #[allow(non_camel_case_types)]
   EOS,
   #[allow(non_camel_case_types)]
   ETC,
   #[allow(non_camel_case_types)]
   ETH,
   #[allow(non_camel_case_types)]
   LTC,
   #[allow(non_camel_case_types)]
   NEO,
   #[allow(non_camel_case_types)]
   XLM,
   #[allow(non_camel_case_types)]
   XMR,
   #[allow(non_camel_case_types)]
   XRP,
   #[allow(non_camel_case_types)]
   ZEC,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MonetaryAmount {
   #[serde(
      rename = "$class",
   )]
   pub _class: String,
   
   #[serde(
      rename = "doubleValue",
   )]
   pub double_value: f64,
   
   #[serde(
      rename = "currencyCode",
   )]
   pub currency_code: CurrencyCode,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum CurrencyCode {
   #[allow(non_camel_case_types)]
   AED,
   #[allow(non_camel_case_types)]
   AFN,
   #[allow(non_camel_case_types)]
   ALL,
   #[allow(non_camel_case_types)]
   AMD,
   #[allow(non_camel_case_types)]
   ANG,
   #[allow(non_camel_case_types)]
   AOA,
   #[allow(non_camel_case_types)]
   ARS,
   #[allow(non_camel_case_types)]
   AUD,
   #[allow(non_camel_case_types)]
   AWG,
   #[allow(non_camel_case_types)]
   AZN,
   #[allow(non_camel_case_types)]
   BAM,
   #[allow(non_camel_case_types)]
   BBD,
   #[allow(non_camel_case_types)]
   BDT,
   #[allow(non_camel_case_types)]
   BGN,
   #[allow(non_camel_case_types)]
   BHD,
   #[allow(non_camel_case_types)]
   BIF,
   #[allow(non_camel_case_types)]
   BMD,
   #[allow(non_camel_case_types)]
   BND,
   #[allow(non_camel_case_types)]
   BOB,
   #[allow(non_camel_case_types)]
   BOV,
   #[allow(non_camel_case_types)]
   BRL,
   #[allow(non_camel_case_types)]
   BSD,
   #[allow(non_camel_case_types)]
   BTN,
   #[allow(non_camel_case_types)]
   BWP,
   #[allow(non_camel_case_types)]
   BYN,
   #[allow(non_camel_case_types)]
   BZD,
   #[allow(non_camel_case_types)]
   CAD,
   #[allow(non_camel_case_types)]
   CDF,
   #[allow(non_camel_case_types)]
   CHE,
   #[allow(non_camel_case_types)]
   CHF,
   #[allow(non_camel_case_types)]
   CHW,
   #[allow(non_camel_case_types)]
   CLF,
   #[allow(non_camel_case_types)]
   CLP,
   #[allow(non_camel_case_types)]
   CNY,
   #[allow(non_camel_case_types)]
   COP,
   #[allow(non_camel_case_types)]
   COU,
   #[allow(non_camel_case_types)]
   CRC,
   #[allow(non_camel_case_types)]
   CUC,
   #[allow(non_camel_case_types)]
   CUP,
   #[allow(non_camel_case_types)]
   CVE,
   #[allow(non_camel_case_types)]
   CZK,
   #[allow(non_camel_case_types)]
   DJF,
   #[allow(non_camel_case_types)]
   DKK,
   #[allow(non_camel_case_types)]
   DOP,
   #[allow(non_camel_case_types)]
   DZD,
   #[allow(non_camel_case_types)]
   EGP,
   #[allow(non_camel_case_types)]
   ERN,
   #[allow(non_camel_case_types)]
   ETB,
   #[allow(non_camel_case_types)]
   EUR,
   #[allow(non_camel_case_types)]
   FJD,
   #[allow(non_camel_case_types)]
   FKP,
   #[allow(non_camel_case_types)]
   GBP,
   #[allow(non_camel_case_types)]
   GEL,
   #[allow(non_camel_case_types)]
   GHS,
   #[allow(non_camel_case_types)]
   GIP,
   #[allow(non_camel_case_types)]
   GMD,
   #[allow(non_camel_case_types)]
   GNF,
   #[allow(non_camel_case_types)]
   GTQ,
   #[allow(non_camel_case_types)]
   GYD,
   #[allow(non_camel_case_types)]
   HKD,
   #[allow(non_camel_case_types)]
   HNL,
   #[allow(non_camel_case_types)]
   HRK,
   #[allow(non_camel_case_types)]
   HTG,
   #[allow(non_camel_case_types)]
   HUF,
   #[allow(non_camel_case_types)]
   IDR,
   #[allow(non_camel_case_types)]
   ILS,
   #[allow(non_camel_case_types)]
   INR,
   #[allow(non_camel_case_types)]
   IQD,
   #[allow(non_camel_case_types)]
   IRR,
   #[allow(non_camel_case_types)]
   ISK,
   #[allow(non_camel_case_types)]
   JMD,
   #[allow(non_camel_case_types)]
   JOD,
   #[allow(non_camel_case_types)]
   JPY,
   #[allow(non_camel_case_types)]
   KES,
   #[allow(non_camel_case_types)]
   KGS,
   #[allow(non_camel_case_types)]
   KHR,
   #[allow(non_camel_case_types)]
   KMF,
   #[allow(non_camel_case_types)]
   KPW,
   #[allow(non_camel_case_types)]
   KRW,
   #[allow(non_camel_case_types)]
   KWD,
   #[allow(non_camel_case_types)]
   KYD,
   #[allow(non_camel_case_types)]
   KZT,
   #[allow(non_camel_case_types)]
   LAK,
   #[allow(non_camel_case_types)]
   LBP,
   #[allow(non_camel_case_types)]
   LKR,
   #[allow(non_camel_case_types)]
   LRD,
   #[allow(non_camel_case_types)]
   LSL,
   #[allow(non_camel_case_types)]
   LYD,
   #[allow(non_camel_case_types)]
   MAD,
   #[allow(non_camel_case_types)]
   MDL,
   #[allow(non_camel_case_types)]
   MGA,
   #[allow(non_camel_case_types)]
   MKD,
   #[allow(non_camel_case_types)]
   MMK,
   #[allow(non_camel_case_types)]
   MNT,
   #[allow(non_camel_case_types)]
   MOP,
   #[allow(non_camel_case_types)]
   MRU,
   #[allow(non_camel_case_types)]
   MUR,
   #[allow(non_camel_case_types)]
   MVR,
   #[allow(non_camel_case_types)]
   MWK,
   #[allow(non_camel_case_types)]
   MXN,
   #[allow(non_camel_case_types)]
   MXV,
   #[allow(non_camel_case_types)]
   MYR,
   #[allow(non_camel_case_types)]
   MZN,
   #[allow(non_camel_case_types)]
   NAD,
   #[allow(non_camel_case_types)]
   NGN,
   #[allow(non_camel_case_types)]
   NIO,
   #[allow(non_camel_case_types)]
   NOK,
   #[allow(non_camel_case_types)]
   NPR,
   #[allow(non_camel_case_types)]
   NZD,
   #[allow(non_camel_case_types)]
   OMR,
   #[allow(non_camel_case_types)]
   PAB,
   #[allow(non_camel_case_types)]
   PEN,
   #[allow(non_camel_case_types)]
   PGK,
   #[allow(non_camel_case_types)]
   PHP,
   #[allow(non_camel_case_types)]
   PKR,
   #[allow(non_camel_case_types)]
   PLN,
   #[allow(non_camel_case_types)]
   PYG,
   #[allow(non_camel_case_types)]
   QAR,
   #[allow(non_camel_case_types)]
   RON,
   #[allow(non_camel_case_types)]
   RSD,
   #[allow(non_camel_case_types)]
   RUB,
   #[allow(non_camel_case_types)]
   RWF,
   #[allow(non_camel_case_types)]
   SAR,
   #[allow(non_camel_case_types)]
   SBD,
   #[allow(non_camel_case_types)]
   SCR,
   #[allow(non_camel_case_types)]
   SDG,
   #[allow(non_camel_case_types)]
   SEK,
   #[allow(non_camel_case_types)]
   SGD,
   #[allow(non_camel_case_types)]
   SHP,
   #[allow(non_camel_case_types)]
   SLL,
   #[allow(non_camel_case_types)]
   SOS,
   #[allow(non_camel_case_types)]
   SRD,
   #[allow(non_camel_case_types)]
   SSP,
   #[allow(non_camel_case_types)]
   STN,
   #[allow(non_camel_case_types)]
   SVC,
   #[allow(non_camel_case_types)]
   SYP,
   #[allow(non_camel_case_types)]
   SZL,
   #[allow(non_camel_case_types)]
   THB,
   #[allow(non_camel_case_types)]
   TJS,
   #[allow(non_camel_case_types)]
   TMT,
   #[allow(non_camel_case_types)]
   TND,
   #[allow(non_camel_case_types)]
   TOP,
   #[allow(non_camel_case_types)]
   TRY,
   #[allow(non_camel_case_types)]
   TTD,
   #[allow(non_camel_case_types)]
   TWD,
   #[allow(non_camel_case_types)]
   TZS,
   #[allow(non_camel_case_types)]
   UAH,
   #[allow(non_camel_case_types)]
   UGX,
   #[allow(non_camel_case_types)]
   USD,
   #[allow(non_camel_case_types)]
   USN,
   #[allow(non_camel_case_types)]
   UYI,
   #[allow(non_camel_case_types)]
   UYU,
   #[allow(non_camel_case_types)]
   UZS,
   #[allow(non_camel_case_types)]
   VEF,
   #[allow(non_camel_case_types)]
   VND,
   #[allow(non_camel_case_types)]
   VUV,
   #[allow(non_camel_case_types)]
   WST,
   #[allow(non_camel_case_types)]
   XAF,
   #[allow(non_camel_case_types)]
   XAG,
   #[allow(non_camel_case_types)]
   XAU,
   #[allow(non_camel_case_types)]
   XBA,
   #[allow(non_camel_case_types)]
   XBB,
   #[allow(non_camel_case_types)]
   XBC,
   #[allow(non_camel_case_types)]
   XBD,
   #[allow(non_camel_case_types)]
   XCD,
   #[allow(non_camel_case_types)]
   XDR,
   #[allow(non_camel_case_types)]
   XOF,
   #[allow(non_camel_case_types)]
   XPD,
   #[allow(non_camel_case_types)]
   XPF,
   #[allow(non_camel_case_types)]
   XPT,
   #[allow(non_camel_case_types)]
   XSU,
   #[allow(non_camel_case_types)]
   XTS,
   #[allow(non_camel_case_types)]
   XUA,
   #[allow(non_camel_case_types)]
   XXX,
   #[allow(non_camel_case_types)]
   YER,
   #[allow(non_camel_case_types)]
   ZAR,
   #[allow(non_camel_case_types)]
   ZMW,
   #[allow(non_camel_case_types)]
   ZWL,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CurrencyConversion {
   #[serde(
      rename = "$class",
   )]
   pub _class: String,
   
   #[serde(
      rename = "from",
   )]
   pub from: CurrencyCode,
   
   #[serde(
      rename = "to",
   )]
   pub to: CurrencyCode,
   
   #[serde(
      rename = "rate",
   )]
   pub rate: f64,
}

