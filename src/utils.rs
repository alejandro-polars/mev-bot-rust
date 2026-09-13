use ethers::prelude::*;

/// Calculate profit after gas costs
pub fn calculate_net_profit(
    gross_profit: U256,
    gas_used: u64,
    gas_price: U256,
) -> U256 {
    let gas_cost = U256::from(gas_used) * gas_price;
    
    if gas_cost > gross_profit {
        U256::zero()
    } else {
        gross_profit - gas_cost
    }
}

/// Format address for logging
pub fn format_address(addr: &Address) -> String {
    format!("0x{}...{}", 
        hex::encode(&addr[..2]),
        hex::encode(&addr[addr.len()-2..])
    )
}

/// Parse token amount from calldata
pub fn parse_amount_from_calldata(data: &[u8], offset: usize) -> Option<U256> {
    if data.len() < offset + 32 {
        return None;
    }
    
    let mut amount_bytes = [0u8; 32];
    amount_bytes.copy_from_slice(&data[offset..offset + 32]);
    Some(U256::from_big_endian(&amount_bytes))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_net_profit() {
        let gross_profit = U256::from(1_000_000_000_000_000_000u64); // 1 ETH
        let gas_used = 200_000u64;
        let gas_price = U256::from(20_000_000_000u64); // 20 Gwei

        let net_profit = calculate_net_profit(gross_profit, gas_used, gas_price);
        let gas_cost = U256::from(gas_used) * gas_price;
        
        assert_eq!(net_profit, gross_profit - gas_cost);
    }

    #[test]
    fn test_calculate_net_profit_loss() {
        let gross_profit = U256::from(100_000_000_000_000u64); // 0.0001 ETH
        let gas_used = 200_000u64;
        let gas_price = U256::from(100_000_000_000u64); // 100 Gwei

        let net_profit = calculate_net_profit(gross_profit, gas_used, gas_price);
        
        assert_eq!(net_profit, U256::zero());
    }
}
