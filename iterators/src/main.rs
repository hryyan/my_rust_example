fn main() {
    for i in (1..100).filter(|&x| x % 2 == 0) {
        println!("{}", i);
    }

    for i in (1..1000).filter(|&x| x % 2 == 0)
                      .filter(|&x| x % 3 == 0)
                          .take(5) {
                              println!("{}", i);
                          }
}

