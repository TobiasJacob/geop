# EFloat Interval Arithmetic

Lets start with a little fun fact. Did you know, that with `f64` floating point numbers, $$0.1 + 0.2 \neq 0.3$$? This is because floating point numbers are not exact. They are approximations. In most programs, it is sufficient to use a small epsilon value to compare floating point numbers. However, in geometric algorithms, we need to be more precise. This is where interval arithmetic comes in.

Geop utilizes the EFloat64 type to represent floating point numbers. It tracks the lower and upper bounds of the floating point number. This allows us to perform arithmetic operations on floating point numbers with a guaranteed error bound. The following testcase magically works. Note how with f64, 0.1 + 0.2 != 0.3, but with EFloat64, 0.1 + 0.2 == 0.3.

```rust
#[test]
fn test_efloat_add() {
    assert!(0.1 + 0.2 != 0.3);
    let a = EFloat64::from(0.1);
    let b = EFloat64::from(0.2);
    let c = a + b;
    println!("c: {:?}", c);
    assert!(c == 0.3);
}
```

Typically, geop will use EFloat64 for all calculations like addition, squareroot, dot products, cross products, etc. which results in an EFloat64 again. As a last step, the EFloat64 is compared to a f64 to determine e.g. if two points intersect. E.g. the following code snipped from the `geop-geometry` checks if two points are equal:

```rust
impl PartialEq for Point {
    fn eq(&self, other: &Point) -> bool {
        (self.x - other.x) == 0.0 && (self.y - other.y) == 0.0 && (self.z - other.z) == 0.0
    }
}
```

So typically, EFloat64 is used for all calculations, and f64 is only used for comparison. This way, we can guarantee that the error bound is always within a certain range.

## Things that return results instead of panicking

There is a couple things which are usually a bad idea:
- Dividing by EFloat64. This returns a AlgebraResult<EFloat64>. Why? Because of division by zero. If you divide by zero, the result is not a number. So we return a result instead of panicking.
- Taking a square root of EFloat64. Well, usually a bad idea, since it is slow, and does not work for negative values.
- Normalizing a Point. This returns a AlgebraResult<Point>. Why? Because if the point is at the origin, it cannot be normalized. So we return a result instead of panicking.

Why does this matter? Take a look for example at this snipped to check if two homogenous points are equal:

```rust
// equality
impl<T> PartialEq for Homogeneous<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        (self.point / self.weight).unwrap() == (other.point / other.weight).unwrap()
    }
}
```

Now this panics, if the points weight is 0. Take a look at this implementation instead

```rust
// equality
impl<T> PartialEq for Homogeneous<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.point * other.weight == other.point * self.weight
    }
}
```

This implementation gets the same result, but without needing a division. Thus, it cannot panic, and the implementation is more robust.

People tend to forget to check these edge cases, thus, geop uses the enhanced typing capabilities of Rust, to remind you that a division, or a normalization operation might fail.
