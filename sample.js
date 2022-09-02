function* gen(){
    yield* [1,4];
    yield* [5,7];
}
const arr = [];
for(const result of gen()){
    arr.push(result);
}
IO(arr.join(","));
