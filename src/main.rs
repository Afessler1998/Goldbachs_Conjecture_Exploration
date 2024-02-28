use std::fs::OpenOptions;
use std::io::Write;

fn is_prime(num: u64) -> bool {
    if num == 1 {
        return false;
    }

    if num == 2 {
        return true;
    }

    for i in 2..(num / 2 + 1) {
        if num % i == 0 {
            return false;
        }
    }

    true
}

fn next_prime(num: u64) -> u64 {
    let mut start = num + 1;
    loop {
        let prime = is_prime(start);
        if prime {
            return start
        }
        start += 1
    }
}

fn main() -> Result<(), std::io::Error> {
    let mut primes: Vec<u64> = vec![2, 3];
    let mut even = 4;
    
    let mut file = OpenOptions::new().write(true).open("output.txt")?;

    loop {
        let mut l = 0;
        let mut r = primes.len() - 1;
        let mut found = false;

        if primes[r] < even {
            primes.push(next_prime(primes[primes.len() - 1]));
            r = primes.len() - 1;
        }

        while l <= r {
            if primes[l] + primes[r] == even {
                let output = format!("{} = {} + {}\n", even, primes[l], primes[r]);
                file.write_all(output.as_bytes())?;
                found = true;
                break;
            } else if primes[l] + primes[r] < even {
                l += 1;
            } else {
                r -= 1;
            }
        }

        if !found {
            let output = format!("No pair found for even = {}\n", even);
            file.write_all(output.as_bytes())?;
        }

        even += 2;
    }

    Ok(())
}