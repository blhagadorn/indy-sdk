println!("7. Build NYM request to add Trust Anchor to the ledger");
let build_nym_request: String = Ledger::build_nym_request(&steward_did, &trustee_did, Some(&trustee_verkey), None, Some("TRUST_ANCHOR")).unwrap();

println!("8. Sending the nym request to ledger");
let _build_nym_sign_submit_result: String = Ledger::sign_and_submit_request(pool_handle, wallet_handle, &steward_did, &build_nym_request).unwrap();