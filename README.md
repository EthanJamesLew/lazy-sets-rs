# LazySets-rs
Implementing the scalable symbolic-numeric set computations ideas of LazySets.jl in Rust. 

## Approach

This summary is based on the LazySets.jl paper. Sets and their operations are represented *lazily*, having them live in the same abstraction layer and returning operation values "by need" using compsable computations. 

### Example: Support Function Evaluation
For example, consider the set $\mathcal Y \subseteq \mathbb R^n$ build from the sets $\mathcal X_0$, $\mathcal U$, and linear transformations $A$, $\delta B$, $e^{A \delta}$, the Minkowksi sum $\oplus$, and the convex hull operator $\operatorname{ConvexHull}$, 

$$
\mathcal Y = \operatorname{ConvexHull} \left( e^{A \delta} \mathcal X_0 \oplus \delta B \mathcal U, \mathcal X_0 \right)
$$

$\mathcal Y$ can be represented with a tree of LazySet objects
```
ConvexHull
├── MinkowskiSum
│   ├── ExponentialMap(A delta)
│   │   └── X_0
│   └── LinearMap(delta B)
│       └── U
└── X_0
```

With this lazy representation, say we want to compute a *support value* for a direction $d \in \mathbb R^n$. The support value is found by solving the problem
$$
g_{\mathcal Y}(d) = \operatorname*{max}_{y \in \mathcal Y} \langle d, y \rangle
$$
Now, we don't know the support vector for this set directly, but say we do know the support vector for the underlying sets $\mathcal X_0$ and $\mathcal U$. Support functions follow the compositional rules

$$
\begin{aligned}
g_{\mathcal X \oplus \mathcal Y} (u) &= g_{\mathcal X}(u) + g_{\mathcal Y}(u) \\
g_{\operatorname{ConvexHull}(\mathcal X, \mathcal Y)} (u) &= \max (g_{\mathcal X}(u), g_{\mathcal Y}(u)) \\
g_{A \mathcal X}(u) &= g_{\mathcal X} (A^T u)
\end{aligned}
$$
Evaulating the tree above, it is clear the support function can be computed by
$$
g_{\mathcal Y} (u) = \max \left(g_{\mathcal X_0}\left( \left( e^{A \delta} \right)^T u \right) + g_{\mathcal U}\left( (\delta B)^T u \right) , g_{\mathcal X_0}(u)\right).
$$

$g_{\mathcal X_0}$ and $g_{\mathcal U}$ can be computed if they are

#### Halfspace Polyhedral Set

$\mathcal X = \lbrace  x | x \in A x \le b \rbrace$, solve the linear program

$$
\begin{aligned}
\max u^T x \\
\text{subject to} \quad A x \le b.
\end{aligned}
$$

#### Unit Ball

$\mathcal X = \lbrace x | \|x\|_2 \le 1 \rbrace$,

$$
g_{\mathcal X}(u) = \|u\|_2.
$$

#### Singleton

$\mathcal X = \lbrace  p \rbrace$,
$$
g_{\mathcal X} (u) = \langle p, u \rangle.
$$

Translating a set by $p$ is equivalent to taking $\mathcal Y \oplus \{p\}$.

#### Line

A line segment through the origin with endpoints $(-a, a)$,  $\mathcal X = \lbrace  t a | t \in [-1, 1] \rbrace$,
$$
g_{\mathcal X}(u) = |\langle u, a \rangle|.
$$

## Rust Implementation
In Rust, this means that all operations and set primitives are implemented as structs with a shared trait `LazySet`. For example, the `ConvexHull` is defined as 

```rust
/// Convex hull of two convex sets.
pub struct ConvexHull<N, const D: usize> {
    /// The first support function (left hand side).
    lhs: Box<dyn LazySet<N, D>>,
    /// The second support function (right hand side).
    rhs: Box<dyn LazySet<N, D>>,
}

...

// implement the LazySet trait for ConvexHull
impl<N, const D: usize> LazySet<N, D> for ConvexHull<N, D>
where
    N: RealField,
{
    fn support(&self, direction: &SVector<N, D>) -> (N, SVector<N, D>) {
        let (d1, p1) = self.lhs.support(direction);
        let (d2, p2) = self.rhs.support(direction);
        if d1 > d2 {
            (d1, p1)
        } else {
            (d2, p2)
        }
    }
}
```

## References

Forets, M., & Schilling, C. (2021). LazySets. jl: Scalable symbolic-numeric set computations. arXiv preprint arXiv:2110.01711.
