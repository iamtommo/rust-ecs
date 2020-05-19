// archetype tests

use crate::archetype::ArchetypeId;

#[test]
fn test_archetype_id_equality() {
    assert_eq!(ArchetypeId { index: 1 }, ArchetypeId { index: 1 });
    assert_ne!(ArchetypeId { index: 1 }, ArchetypeId { index: 2 });
}