use crate::*;

// will be removed eventually in favor of problem/solution

pub fn run<T, U, V, FParse, F1, F2>(parse: FParse, part1: F1, part2: F2) -> Result<()>
where
    U: Display,
    V: Display,
    FParse: Fn(&str) -> Result<T>,
    F1: Fn(&T) -> Result<U>,
    F2: Fn(&T) -> Result<V>,
{
    let (input, parse_time) = read_and_parse(parse)?;

    let part1_time = print_and_time("Part 1", || part1(&input)).context("failure in part 1")?;
    let part2_time = print_and_time("Part 2", || part2(&input)).context("failure in part 2")?;

    print_stats(parse_time, part1_time, part2_time);
    Ok(())
}

pub fn run_raw<U, V, F1, F2>(part1: F1, part2: F2) -> Result<()>
where
    U: Display,
    V: Display,
    F1: Fn(&str) -> Result<U>,
    F2: Fn(&str) -> Result<V>,
{
    let (input, parse_time) = read_and_parse(|x| Ok(trim(x)))?;

    let part1_time = print_and_time("Part 1", || part1(&input))?;
    let part2_time = print_and_time("Part 2", || part2(&input))?;

    print_stats(parse_time, part1_time, part2_time);

    Ok(())
}

pub fn run_vec<T, U, V, FParse, F1, F2>(parse: FParse, part1: F1, part2: F2) -> Result<()>
where
    U: Display,
    V: Display,
    FParse: Fn(&str) -> Result<Vec<T>>,
    F1: Fn(&[T]) -> Result<U>,
    F2: Fn(&[T]) -> Result<V>,
{
    let (input, parse_time) = read_and_parse(parse)?;

    let part1_time = print_and_time("Part 1", || part1(&input)).context("failure in part 1")?;
    let part2_time = print_and_time("Part 2", || part2(&input)).context("failure in part 2")?;

    print_stats(parse_time, part1_time, part2_time);
    Ok(())
}

pub fn run_progressive<T, T2, U, V, FParse, F1, F2>(
    parse: FParse,
    part1: F1,
    part2: F2,
) -> Result<()>
where
    U: Display,
    V: Display,
    FParse: Fn(&str) -> Result<T>,
    F1: Fn(&T) -> Result<(U, T2)>,
    F2: Fn(&T, &T2) -> Result<V>,
{
    let (input, parse_time) = read_and_parse(parse)?;

    let (part1_time, data_for_next) = print_and_time_and_return("Part 1", || part1(&input))?;
    let part2_time = print_and_time("Part 2", || part2(&input, &data_for_next))?;

    print_stats(parse_time, part1_time, part2_time);

    Ok(())
}

pub fn run_progressive_vec<T, T2, U, V, FParse, F1, F2>(
    parse: FParse,
    part1: F1,
    part2: F2,
) -> Result<()>
where
    U: Display,
    V: Display,
    FParse: Fn(&str) -> Result<Vec<T>>,
    F1: Fn(&[T]) -> Result<(U, T2)>,
    F2: Fn(&[T], &T2) -> Result<V>,
{
    let (input, parse_time) = read_and_parse(parse)?;

    let start = Instant::now();
    let (result, data_for_next) = part1(&input).context("failure in part 1")?;
    let part1_time = start.elapsed();

    print!("Part 1 - ");
    let result = format!("{result}");
    if result.len() > 20 || result.contains('\n') {
        println!();
    }
    println!("{}", style(result).bold());

    let part2_time =
        print_and_time("Part 2", || part2(&input, &data_for_next)).context("failure in part 2")?;

    print_stats(parse_time, part1_time, part2_time);

    Ok(())
}
