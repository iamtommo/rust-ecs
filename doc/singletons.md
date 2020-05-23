Design mirrors that of Unity's Singleton components however has no concept of an entity

singleton storage is a simple hashmap in Universe
there's no create method rather only set/get/has

subsequent set_singleton calls will simply overwrite existing data