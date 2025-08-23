pub fn echo(args: &[String]) {
    if args.len() == 0 {
        println!();
        return;
    }

    let output = args.join(" ");

    let output = output.replace("\\n", "\n").replace("\\t", "\t");
    println!("{}", output);
}
