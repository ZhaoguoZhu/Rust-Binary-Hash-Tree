use std::io;

fn main() {
    println!("Hello, world!");

    let mut input = String::new();
    println!("type in number of words");
    io::stdin().read_line(&mut input )
        .expect("Failed to read line");

    let input: u32 = input.trim().parse()
        .expect("Please type a number!");
    println!("You entered: {}", input);
    if(input != 0){
        let mut v = create_random_vector(input);
        print_vector(&v);

        hm_tree_wrapper(&v);

        let mut tam = String::new();
        println!("type in index of word to be changed");
        io::stdin().read_line(&mut tam )
            .expect("Failed to read line");

        let tam: usize = tam.trim().parse()
            .expect("Please type a number!");
        println!("You entered: {}", tam);
        v[tam] = "changed".to_string();
        hm_tree_wrapper(&v);

    }else{
        let mut root = Node { hash: String::from("root node"),
                                name: String::from("root node"),
                                left_c: None,
                                right_c: None,} ;

        let mut lc = Node { hash: String::from("left node"),
                                name: String::from("left node"),
                                left_c: None,
                                right_c: None,} ;

        let mut rc = Node { hash: String::from("right node"),
                                name: String::from("right node"),
                                left_c: None,
                                right_c: None,} ;

        root.left_c = Some(Box::new(lc));
        root.right_c = Some(Box::new(rc));
        printtree(&root);
    }

    //tamper(&mut v, tam);

    //let y = String::from("abcd");
    //let x = primitive_hash_fn(&y); testing hash functions
    //println!("{}", x);

    //let mut x: Vec<String> = Vec::new();
    //x.push("really".to_string()); found out without trim(), input strings would print in new line
    //x.push("why".to_string());
    //print_vector(&x);


}

struct Node {
    hash: String,
    name: String,
    left_c: Option<Box<Node>>,
    right_c: Option<Box<Node>>

}

impl Clone for Node {
    //fn clone(&self) -> Node {*self}
    fn clone(&self) -> Node {
        let mut t_lc = None;
        match &self.left_c {
            Some(p)=> t_lc = Some(Box::new(*p.clone())),
            None => t_lc = None,

        }

        let mut r_lc = None;
        match &self.right_c {
            Some(p)=> r_lc = Some(Box::new(*p.clone())),
            None => r_lc = None,

        }
        let temp = Node {hash: self.hash.clone(),
        name: self.name.clone(),

        left_c: t_lc,
        right_c: r_lc,};

        temp
    }
}

fn printtree(xor: &Node,){ //in_order

    let left_traverse = &xor.left_c;
    match left_traverse {
        Some(p)=> printtree(&*p),
        None => (),

    }
    println!("{}",xor.hash);
    let right_traverse = &xor.right_c;
    match right_traverse {
        Some(p)=> printtree(&*p),
        None => (),

    }

}

fn tamper(xor: &mut Vec<String>, i: usize){

    let num = xor.len();
    let mut temp = String::new();
    println!("type in word");
    io::stdin().read_line(&mut temp )
        .expect("Failed to read line");

    for n in 0..num {
        if n == i {
            xor[n] = temp.trim().to_string();
        }
    }
}

fn hm_tree_wrapper(xor: &Vec<String>) {
    let mut temp = to_hash(xor);
    let node = hm_tree(&mut temp);
    printtree(&node);
    print_vector(xor);
}

fn to_hash(xor: &Vec<String>) -> Vec<Node>{
    let num = xor.len();
    let mut trial: Vec<Node> = Vec::new();
    for n in 0..num {
        let mut root = Node { hash: primitive_hash_fn(&xor[n]),
                                name: String::from(n.to_string()),
                                left_c: None,
                                right_c: None,} ;
        trial.push(root);

    }
    trial
}

fn to_hash_con(xor: &Vec<Node>) -> Vec<Node>{
    let num = xor.len();
    let mut trial: Vec<Node> = Vec::new();
    if(num == 1){
        trial = xor.to_vec();
    }else {
        let mut n = 0;
        while n < num {
            let mut temp = String::new();
            temp.push_str(&xor[n].hash);
            temp.push_str(&xor[n+1].hash);
            let nhash = primitive_hash_fn(&temp);

            let mut nn = String::new();
            nn.push_str(&xor[n].name);
            nn.push_str(&xor[n+1].name);

            //let mut lc = Node { hash: xor[n].hash.clone(),
                //name: xor[n].name.clone(),
                //left_c: xor[n].left_c.copied(),
                //right_c: xor[n].right_c.copied(),

            //};

            //let mut lc = Node {..xor[n]};
            //let mut rc = Node {..xor[n+1]};
            let mut lc = xor[n].clone();
            let mut rc = xor[n+1].clone();
            let mut root = Node { hash: nhash,
                                    name: String::from(nn.to_string()),
                                    left_c: Some(Box::new(lc)),
                                    right_c: Some(Box::new(rc)),} ;
            //try creating a copy of nodes in Xor to place in to the new node (brand new duplicate)
            trial.push(root);
            n = n + 2;
            if(n == num-1){

                trial.push(xor[n].clone());
                n = n + 1;
            }
        }
        trial = to_hash_con(&trial);
    }

    trial
}

fn hm_tree(mut xor: &Vec<Node>) -> Node{
    let mut n = xor.len();
    let mut temp = to_hash_con(&xor);
    //while n > 1 {
        //temp = to_hash_con(&xor);
        //n = xor.len();
    //}
    let root = temp[0].clone();

    //if n > 1 {
        //let temp = to_hash_con(xor);
        //hm_tree(&temp);
        //print_vector(xor);
    //} else {
        //print!("The root hash:  ");
        //print_vector(xor);
    //}
    root
}

fn primitive_hash_fn(input: &str) -> String {
    let mut buf = String::with_capacity(4);
    let mut holder = 0;
    let mut ind = 0;
    for c in input.chars() {
        let x = c.to_digit(36);
        ind += 1;
        match x {
            None => (),
            Some(temp) => holder = holder + (temp * ind),
            }
   }
   holder = holder % 2099;
   let temp = holder.to_string();
   buf.push_str(&temp);
   while buf.len() < 4 {
      buf.push('z');
   }
   buf
}

fn create_random_vector(input: u32) -> Vec<String> {
    let mut trial: Vec<String> = Vec::new();

    for n in 0..input {
        let mut temp = String::new();
        println!("type in word");
        io::stdin().read_line(&mut temp )
            .expect("Failed to read line");
        trial.push(temp.trim().to_string());
    }
    trial
}

fn print_vector(xor: &Vec<String>) {
    let num = xor.len();
    for n in 0..num {
        print!("{} ",xor[n]);
    }
    println!("");
}
