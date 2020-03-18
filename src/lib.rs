use std::collections::HashSet;

/// Assumptions:
/// Each passed email contains exactly one '@' character.
/// All local and domain names are non-empty.
/// Local names do not start with a '+' character.
pub fn unique_email_addresses(emails: Vec<String>) -> u32 {
    let mut unique_addresses = HashSet::new();
    for email in &emails {
        let local_domain_split: Vec<&str> = email.split("@").collect();
        let mut local = local_domain_split[0];
        let local_plus_char_index = local.find("+");
        if let Some(index) = local_plus_char_index {
            local = &local[0..index];
        }
        let local_portion = &local.split(".").collect::<Vec<&str>>().join("");
        unique_addresses.insert([local_portion, local_domain_split[1]].join("@"));
    }
    unique_addresses.len() as u32
}

#[cfg(test)]
mod tests {
    use super::{unique_email_addresses};
    #[test]
    fn test_unique_email_addresses() {
        let emails = vec![
            String::from("vartan.vartabed.kalustian+email@gmail.com"),
            String::from("vartanvartabedkalustian+email@gmail.com"),
            String::from("vartan.vartabedkalustian+email@gmail.com"),
            String::from("vartanvartabedkalustian@gmail.com"),
            String::from("vartanvartabed.kalustian@gmail.com"),
        ];
        assert_eq!(unique_email_addresses(emails), 1);
    }
}
