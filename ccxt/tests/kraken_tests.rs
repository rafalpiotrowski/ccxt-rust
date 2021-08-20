
mod kraken_tests
{
    use ccxt::kraken::*;
    use ccxt::exchange::*;

    #[test]
    /*
    Example Signature

The following is a specific example of a signature generated with a particular private key, nonce, and payload corresponding to a new limit order (buy 1.25 XBTUSD at $37,500). If your code is generating a different signature ("API-Sign") for thie example, then there is likely an issue with your application of the above methodology. Code snippets for generating the signature in Python, Golang and Node.js follow below.
Field 	Value
Private Key 	kQH5HW/8p1uGOVjbgWA7FunAmGO8lsSUXNsu3eow76sz84Q18fWxnyRzBHCd3pd5nE9qa99HAZtuZuj6F1huXg==
Nonce 	1616492376594
Encoded Payload 	nonce=1616492376594&ordertype=limit&pair=XBTUSD&price=37500&type=buy&volume=1.25
URI Path 	/0/private/AddOrder
API-Sign 	4/dpxb3iT4tp/ZCVEwSnEsLxx0bqyhLpdfOpc6fn7OR8+UClSV5n9E6aSS8MPtnRfp32bAb0nmbRn6H8ndwLUQ==
     */
    fn exmple_signature() {
        let k = Kraken::new("1", "".to_string(), "kQH5HW/8p1uGOVjbgWA7FunAmGO8lsSUXNsu3eow76sz84Q18fWxnyRzBHCd3pd5nE9qa99HAZtuZuj6F1huXg==".to_string());
        let uri_path = "/0/private/AddOrder".to_string();
        let post_data = "nonce=1616492376594&ordertype=limit&pair=XBTUSD&price=37500&type=buy&volume=1.25".to_string();
        let nonce = "1616492376594".to_string();
        let s = k.get_signature(&uri_path, &post_data, &nonce);
        assert_eq!("4/dpxb3iT4tp/ZCVEwSnEsLxx0bqyhLpdfOpc6fn7OR8+UClSV5n9E6aSS8MPtnRfp32bAb0nmbRn6H8ndwLUQ==".to_string(), s);
    }
}