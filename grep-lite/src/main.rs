fn main() {
    let ctx_lines = 2;
    let needle = "oo";
    let haystack = "\
Every face, every shop,
bedroom window, public-house, and
dark square is a picture
feverishly turned--in search of what?
It is the same with books.
What do we seek
through millions of pages?";

    let mut tags: Vec<usize> = vec![];
    let mut ctx: Vec<Vec<String>> = vec![];

    for (i, line) in haystack.lines().enumerate() {
        // <3>
        if line.contains(needle) {
            tags.push(i);

            ctx.push(Vec::with_capacity(2 * ctx_lines + 1));
        }
    }

    if tags.is_empty() {
        return;
    }

    for (i, line) in haystack.lines().enumerate() {
        // <6>
        for (j, tag) in tags.iter().enumerate() {
            let lower_bound = tag.saturating_sub(ctx_lines);
            let upper_bound = tag + ctx_lines;

            if (i >= lower_bound) && (i <= upper_bound) {
                if i == *tag {
                    ctx[j].push(format!("{}:\t{}", i, String::from(line)));
                } else {
                    ctx[j].push(format!("{}-\t{}", i, String::from(line)));
                }
            }
        }
    }

    for (i, local_ctx) in ctx.iter().enumerate() {
        if i > 0 {
            println!("--");
        }

        for x in local_ctx.iter() {
            println!("{}", x);
        }
    }
}
