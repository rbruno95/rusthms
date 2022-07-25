use rusthms::algorithms::graphs::trie::Trie;

#[test]
pub fn it_should_be_able_to_insert_and_search_a_word_in_a_trie() {
    let mut trie = Trie::new();
    trie.insert("hello");
    assert_eq!(trie.search("hello"), true);
    assert_eq!(trie.search("hell"), false);
    trie.insert("hell");
    assert_eq!(trie.search("hell"), true);
    trie.insert("shadow");
    assert_eq!(trie.search("shadow"), true);
    assert_eq!(trie.search("shadowy"), false);
    trie.insert("shader");
    assert_eq!(trie.search("shader"), true);
    assert_eq!(trie.search("shade"), false);
    assert_eq!(trie.search("shad"), false);
}
