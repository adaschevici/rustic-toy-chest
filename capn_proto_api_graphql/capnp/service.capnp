@0xeb1bbbd418f18514;

interface Job {
    register @0 Request -> RegistryItem;
    getResult @1 RegistryItem -> Result;
    struct Request {
        parms @0 :List(Text);
        source @1 :Text;
    }
    struct Result {
        union {
            link @0 :Text;
            error @1 :Text;
        }
    }
    struct RegistryItem {
        # Single Job Identity
        id @0 :Text;
    }
}
