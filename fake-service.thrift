service FakeThingy {
    void ping(),
    i32 add(1: required i32 num1, 2: required i32 num2),
    double divide(1: required double num1, 2: required double num2) throws (1: InvalidOperation ouch),
    FakeProfile get_profile(1: required i32 id)
}

exception InvalidOperation {
    1: required i32 code;
    2: required string why;
}

struct FakeProfile {
    1: required i32 id;
    2: string first_name;
    3: string last_name;
    4: string email;
}
