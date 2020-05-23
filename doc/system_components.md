Design mirrors that of Unity's SystemStateComponents

usage:
store state specific to a system for user entities


semantics:
if a user deletes an entity with SystemComponents attached, SystemComponents will not be deleted and the entity
will not actually be destroyed. This gives the system a chance to cleanup system state by i.e. querying for
entities which have a specific systemcomponent but no user data attached.

notes:
not yet decided at which point entities are _actually_ deleted.
possibly when the _last_ SystemComponent is removed from an entity?
or should each system which touches a SystemComponent removal explicitly call destroy entity?