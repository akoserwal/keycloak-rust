struct  Client {
    realm: String,
    id: String,
    cred: Credentials,
}

struct Credentials {
    secret: String
}