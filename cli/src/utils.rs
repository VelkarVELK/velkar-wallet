use crate::error::Error;
use crate::result::Result;
use velkar_consensus_core::constants::SOMPI_PER_VELKAR;
use std::fmt::Display;

pub fn try_parse_required_nonzero_velkar_as_sompi_u64<S: ToString + Display>(velkar_amount: Option<S>) -> Result<u64> {
    if let Some(velkar_amount) = velkar_amount {
        let sompi_amount = velkar_amount
            .to_string()
            .parse::<f64>()
            .map_err(|_| Error::custom(format!("Supplied Velkar amount is not valid: '{velkar_amount}'")))?
            * SOMPI_PER_VELKAR as f64;
        if sompi_amount < 0.0 {
            Err(Error::custom("Supplied Velkar amount is not valid: '{velkar_amount}'"))
        } else {
            let sompi_amount = sompi_amount as u64;
            if sompi_amount == 0 {
                Err(Error::custom("Supplied required VELK amount must not be a zero: '{velkar_amount}'"))
            } else {
                Ok(sompi_amount)
            }
        }
    } else {
        Err(Error::custom("Missing Velkar amount"))
    }
}

pub fn try_parse_required_velkar_as_sompi_u64<S: ToString + Display>(velkar_amount: Option<S>) -> Result<u64> {
    if let Some(velkar_amount) = velkar_amount {
        let sompi_amount = velkar_amount
            .to_string()
            .parse::<f64>()
            .map_err(|_| Error::custom(format!("Supplied Velkar amount is not valid: '{velkar_amount}'")))?
            * SOMPI_PER_VELKAR as f64;
        if sompi_amount < 0.0 {
            Err(Error::custom("Supplied Velkar amount is not valid: '{velkar_amount}'"))
        } else {
            Ok(sompi_amount as u64)
        }
    } else {
        Err(Error::custom("Missing Velkar amount"))
    }
}

pub fn try_parse_optional_velkar_as_sompi_i64<S: ToString + Display>(velkar_amount: Option<S>) -> Result<Option<i64>> {
    if let Some(velkar_amount) = velkar_amount {
        let sompi_amount = velkar_amount
            .to_string()
            .parse::<f64>()
            .map_err(|_e| Error::custom(format!("Supplied Velkar amount is not valid: '{velkar_amount}'")))?
            * SOMPI_PER_VELKAR as f64;
        if sompi_amount < 0.0 {
            Err(Error::custom("Supplied Velkar amount is not valid: '{velkar_amount}'"))
        } else {
            Ok(Some(sompi_amount as i64))
        }
    } else {
        Ok(None)
    }
}

