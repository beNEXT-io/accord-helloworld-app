use serde::{ Deserialize, Serialize };
use chrono::{ DateTime, TimeZone, Utc };
   
use crate::lib::concerto_1_0_0::*;
use crate::lib::utils::*;
   
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
   ADA,
   BCH,
   BTC,
   DASH,
   EOS,
   ETC,
   ETH,
   LTC,
   NEO,
   XLM,
   XMR,
   XRP,
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
   AED,
   AFN,
   ALL,
   AMD,
   ANG,
   AOA,
   ARS,
   AUD,
   AWG,
   AZN,
   BAM,
   BBD,
   BDT,
   BGN,
   BHD,
   BIF,
   BMD,
   BND,
   BOB,
   BOV,
   BRL,
   BSD,
   BTN,
   BWP,
   BYN,
   BZD,
   CAD,
   CDF,
   CHE,
   CHF,
   CHW,
   CLF,
   CLP,
   CNY,
   COP,
   COU,
   CRC,
   CUC,
   CUP,
   CVE,
   CZK,
   DJF,
   DKK,
   DOP,
   DZD,
   EGP,
   ERN,
   ETB,
   EUR,
   FJD,
   FKP,
   GBP,
   GEL,
   GHS,
   GIP,
   GMD,
   GNF,
   GTQ,
   GYD,
   HKD,
   HNL,
   HRK,
   HTG,
   HUF,
   IDR,
   ILS,
   INR,
   IQD,
   IRR,
   ISK,
   JMD,
   JOD,
   JPY,
   KES,
   KGS,
   KHR,
   KMF,
   KPW,
   KRW,
   KWD,
   KYD,
   KZT,
   LAK,
   LBP,
   LKR,
   LRD,
   LSL,
   LYD,
   MAD,
   MDL,
   MGA,
   MKD,
   MMK,
   MNT,
   MOP,
   MRU,
   MUR,
   MVR,
   MWK,
   MXN,
   MXV,
   MYR,
   MZN,
   NAD,
   NGN,
   NIO,
   NOK,
   NPR,
   NZD,
   OMR,
   PAB,
   PEN,
   PGK,
   PHP,
   PKR,
   PLN,
   PYG,
   QAR,
   RON,
   RSD,
   RUB,
   RWF,
   SAR,
   SBD,
   SCR,
   SDG,
   SEK,
   SGD,
   SHP,
   SLL,
   SOS,
   SRD,
   SSP,
   STN,
   SVC,
   SYP,
   SZL,
   THB,
   TJS,
   TMT,
   TND,
   TOP,
   TRY,
   TTD,
   TWD,
   TZS,
   UAH,
   UGX,
   USD,
   USN,
   UYI,
   UYU,
   UZS,
   VEF,
   VND,
   VUV,
   WST,
   XAF,
   XAG,
   XAU,
   XBA,
   XBB,
   XBC,
   XBD,
   XCD,
   XDR,
   XOF,
   XPD,
   XPF,
   XPT,
   XSU,
   XTS,
   XUA,
   XXX,
   YER,
   ZAR,
   ZMW,
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

