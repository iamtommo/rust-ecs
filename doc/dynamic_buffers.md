cheap solution to the equivalent of Unity's DynamicBuffer i.e. dynamic-length data per entity

since components must be fixed size, cannot store dynamically sized containers
and simply storing a pointer kind of defeats rust's safety guarantees

so the idea is to store a 'handle' to a Vec in components when dynamic storage is needed