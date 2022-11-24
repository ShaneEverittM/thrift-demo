struct Count {
 1: i32 value;
}

service Counter {
    Count increment(1:Count count)
}