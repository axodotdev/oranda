use oranda;

#[test]
fn it_works() {
    assert_eq!(oranda::create_html("# hello"), ">o_o< Hello oranda!");
}
