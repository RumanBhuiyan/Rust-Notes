#![allow(unused)]

/*
    Borrowing rules says that, in one scope there may have multiple immutable borrowers but
        only one mutable borrower. That's why i took mb1 and mb2 mutable borrowers into two
        scopes.

    N.B => If a variable borrows immutably then it can access value until any variable borrows
            mutably. If another variable borrows mutably then value can be accessed by owner
            and mutable borrowers but not immutable borrowers. It also make sense, right ?
            when we borrowed immutably then owners value wasn't changed by mutable borrower but
            after initialization of mutable borrower, owners value may be changed so we shouldn't
            access owners value by immutable borrower again as it will provide unpredicted result
            as owners value has been changed by its mutable borrower.

    mutable and immutable borrows can be explained using this quote of mine
        " multiple readers are reading or single writer is writing but
          readers can't read when writer writes only writer can read"

    So a a variable can have :
                               i) No borrowing
                                        or
                               ii) single immutable borrower
                                        or
                               iii) multiple immutable borrower
                                        or
                               iv) single mutable borrower

        a variable can't have :
                                i) multiple mutable borrower
                                           or
                                ii) immutable borrowers and mutable borrower in same scope

      after immutable borrow, both owner and immutable borrower can access the resource but
      after mutable borrow, resources can only be accessed by mutable borrower.

      Those rules are adopted by Rust to avoid data race conditions in concurrent programming and
       to prevent accessing dangling pointer or invalid address.
 */

pub fn main(){
    let mut num = 12;

    // immutable borrow
    let borrower1 = &num;
    let borrower2 = &num;
    println!("borrower1 : {}", borrower1);
    println!("borrower2 : {}", borrower2);

    // mutable borrow
    {
        let mut mb1 = &mut num;
        *mb1 += 1;
        println!("mut_borrower1 : {}",mb1);
    }
    println!("num : {}",num);
    {
        let mut mb2 = &mut num;
        *mb2 += 1;
        println!("mut_borrower2 : {}",mb2);
    }
    println!("num : {}",num);
}