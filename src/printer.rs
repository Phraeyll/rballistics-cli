use self::Adjustment::*;

use point_mass_ballistics::{
    foot_per_second, foot_pound, inch, moa, second, yard, Length, Measurements,
};

// pub mod plain;
// pub mod pretty;

#[derive(Clone, Copy)]
pub enum Adjustment {
    Elevation(Length),
    Windage(Length),
}

// Show needed adjustments to correct shot
impl Adjustment {
    pub fn adjustment(&self, tolerance: Length) -> char {
        let (n, positive, negative) = match *self {
            Self::Elevation(n) => (n, 'D', 'U'),
            Self::Windage(n) => (n, 'L', 'R'),
        };
        if n > tolerance {
            positive
        } else if n < -tolerance {
            negative
        } else {
            '*'
        }
    }
}

pub fn print<I>(table: I, output_tolerance: Length, pretty: bool)
where
    I: IntoIterator,
    <I as IntoIterator>::Item: Measurements,
{
    let (div, lpad, rpad) = if pretty {
        (
            "+--------------+----------+---------------+-------------+------------+------------+----------------+--------------+----------+\n",
            "| ",
            " |",
        )
    } else {
        ("", "", "")
    };
    println!(
        "{div}{lpad}{:>12} {lpad}{:>8} {lpad}{:>13} {lpad}{:>11} {lpad}{:>10} {lpad}{:>10} {lpad}{:>14} {lpad}{:>12} {lpad}{:>8}{rpad}",
        "Distance(yd)",
        "MOA",
        "Elevation(in)",
        "Windage(in)",
        "Vertical",
        "Horizontal",
        "Velocity(ft/s)",
        "Energy(ftlb)",
        "Time(s)",
        lpad=lpad,
        rpad=rpad,
        div=div,
    );
    for p in table.into_iter() {
        println!(
            "{div}{lpad}{:>12.0} {lpad}{:>8.2} {lpad}{:>11.2} {} {lpad}{:>9.2} {} {lpad}{:>8.2} {} {lpad}{:>8.2} {} {lpad}{:>14.2} {lpad}{:>12.2} {lpad}{:>8.3}{rpad}",
            p.distance().get::<yard>(),
            p.angle().get::<moa>(),
            p.elevation().get::<inch>().abs(),
            Elevation(p.elevation()).adjustment(output_tolerance),
            p.windage().get::<inch>().abs(),
            Windage(p.windage()).adjustment(output_tolerance),
            p.vertical_angle(output_tolerance).get::<moa>().abs(),
            Elevation(p.elevation()).adjustment(output_tolerance),
            p.horizontal_angle(output_tolerance).get::<moa>().abs(),
            Windage(p.windage()).adjustment(output_tolerance),
            p.velocity().get::<foot_per_second>(),
            p.energy().get::<foot_pound>(),
            p.time().get::<second>(),
            lpad=lpad,
            rpad=rpad,
            div=div,
        );
    }
    print!("{}", div);
}
