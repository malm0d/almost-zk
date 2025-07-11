# Almost ZK
This is a simple implementation of an algorithm that shows how to construct an "almost" ZK circuit using R1CS. By "almost", it means that the implementation enforces the correct computation of constraints but lacks full zero-knowledge properties since the witness is exposed and it does not provide privacy guarantees of ZK proofs.

As highlighted by the material from RareSkills, this algorithm is academic and should not be used in any form of production.

This example is built using Rust, and is based on the second module of RareSkill's Book of Zero Knowledge, chapter 3: [Building a Zero Knowledge Proof from an R1CS](https://www.rareskills.io/post/r1cs-zkp).

## Prerequisites
- [Elliptic Curves over Finite Fields](https://www.rareskills.io/post/elliptic-curves-finite-fields)
- [Bilinear Pairings](https://www.rareskills.io/post/bilinear-pairing) (which also means knowing [Group Theory](https://www.rareskills.io/post/group-theory) and [Homomorphism](https://www.rareskills.io/post/homomorphisms))
- [Rank 1 Constraint Systems](https://www.rareskills.io/post/rank-1-constraint-system)

## The R1CS
We want to prove the claim that we know the `x` and `y` values that satisfy:

```math
z = 2x^{3} + 4xy^{2} - xy + 5
```

For the above polynomial, we can break it down to the following set of constraints:

```math
\begin{align}
v_1 = x * x \\
v_2 = 2 v_1 * x \\
v_3 = y * y \\
v_4 = 4x * v_3 \\
5 - z + v_2 + v_4 = x * y
\end{align}
```

The system of equations satisfy the requirements of an R1CS. That is, every constraint has a single non-constant multiplication, and our system of equations are in the form:

```math
\mathbf{Oa} = \mathbf{La} \circ \mathbf{Ra}
```

Where $\mathbf{a}$ is the witness; and $\mathbf{O}$, $\mathbf{L}$, $\mathbf{R}$ are matrices encoding the result coefficients (left side of equality), the left coefficients (left side of multiplication), and the right coefficients (right side of multiplication), respectively for each constraint.

The witness vector is:

$$
\mathbf{a} = \begin{bmatrix}
1 & z & x & y & v_1, & v_2 & v_3 & v_4
\end{bmatrix} \\\\
$$

And $\mathbf{Oa} = \mathbf{La} \circ \mathbf{Ra}$ is:

```math
\begin{bmatrix}
0 & 0 & 0 & 0 & 1 & 0 & 0 & 0 \\
0 & 0 & 0 & 0 & 0 & 1 & 0 & 0 \\
0 & 0 & 0 & 0 & 0 & 0 & 1 & 0 \\
0 & 0 & 0 & 0 & 0 & 0 & 0 & 1 \\
5 & -1 & 0 & 0 & 0 & 1 & 0 & 1 \\
\end{bmatrix}

\begin{bmatrix}
1 \\
z \\
x \\
y \\
v_1 \\
v_2 \\
v_3 \\
v_4 \\
\end{bmatrix} =

\begin{bmatrix}
0 & 0 & 1 & 0 & 0 & 0 & 0 & 0 \\
0 & 0 & 0 & 0 & 2 & 0 & 0 & 0 \\
0 & 0 & 0 & 1 & 0 & 0 & 0 & 0 \\
0 & 0 & 4 & 0 & 0 & 0 & 0 & 0 \\
0 & 0 & 1 & 0 & 0 & 0 & 0 & 0 \\
\end{bmatrix}

\begin{bmatrix}
1 \\
z \\
x \\
y \\
v_1 \\
v_2 \\
v_3 \\
v_4 \\
\end{bmatrix}\circ

\begin{bmatrix}
0 & 0 & 1 & 0 & 0 & 0 & 0 & 0 \\
0 & 0 & 1 & 0 & 0 & 0 & 0 & 0 \\
0 & 0 & 0 & 1 & 0 & 0 & 0 & 0 \\
0 & 0 & 0 & 0 & 0 & 0 & 1 & 0 \\
0 & 0 & 0 & 1 & 0 & 0 & 0 & 0 \\
\end{bmatrix}

\begin{bmatrix}
1 \\
z \\
x \\
y \\
v_1 \\
v_2 \\
v_3 \\
v_4 \\
\end{bmatrix}
```

The multiplication of each matrix with $\mathbf{a}$ is standard matrix multiplication and this helps to associate each term in the witness with their respecitve coefficients. And the $\circ$ operator represents the hadamard product between $\mathbf{La}$ and $\mathbf{Ra}$ which enforces the one multiplication per constraint requirement in R1CS.

In R1CS, all operations are performed in modular arithmetic.

## Proving
Given our witness vector, we want to prove that it satisfies the R1CS without us having to directly reveal it during the verification step. While this example is not fully zero-knowledge, we multiply each term in the witness vector with $G_1$ or $G_2$ to convert witness terms into elliptic curve points. This hides the actual scalar values of the witness, but more importantly, it prepares the witness for bilinear pairing compatibility since the verification step relies on bilinear pairing of elliptic curve points.

Our "encrypted" witness would look something like this:

$$
\mathbf{aG} = \begin{bmatrix}
1G & zG & xG & yG & v_1G, & v_2G & v_3G & v_4G
\end{bmatrix} \\\\
$$

Where $G$ represents the $G_1$ group or $G_2$ group.

The `Bn254` curve used by Ethereum supports the pairing: $e: G_1 \times G_2 \rightarrow \ G_{1 \ 2}$. Without going into detail (leaving all the nitty gritty to RareSkills's chapter on bilinear pairings), the output of a bilinear pairing between elliptic curve groups $G_1$ and $G_2$ is the multiplicative cyclic group of a finite field $G_{1 \ 2}$.

During the verification step, we want to check:

$$
e(L \cdot aG_1, R \cdot aG_2) \stackrel{?}{=} e(O \cdot aG_1, G_2)
$$

And this works because by bilinearity, both sides yield elements in $G_{1 \ 2}$ :

$$
e(aG_1, aG_2)^{L \cdot R} \stackrel{?}{=} e(aG_1, G_2)^{O}
$$

And this holds if and only if $L \cdot R = O$, as required by the R1CS.

## Constraint Encoding through Elliptic Curve Points
To verify the R1CS constraints, we want to lift each linear combination (the dot-product of each matrix with the witness) onto the elliptic curve (i.e. as elliptic curve points). This means we compute the following:

```math
LaG_1 = \begin{bmatrix}
0 & 0 & 1 & 0 & 0 & 0 & 0 & 0 \\
0 & 0 & 0 & 0 & 2 & 0 & 0 & 0 \\
0 & 0 & 0 & 1 & 0 & 0 & 0 & 0 \\
0 & 0 & 4 & 0 & 0 & 0 & 0 & 0 \\
0 & 0 & 1 & 0 & 0 & 0 & 0 & 0 \\
\end{bmatrix} \cdot
\begin{bmatrix}
1G_1 \\
zG_1 \\
xG_1 \\
yG_1 \\
v_1G_1 \\
v_2G_1 \\
v_3G_1 \\
v_4G_1 \\
\end{bmatrix}
=
\left[ \begin{array}{c}
(\sum_{i=1}^m L_{1,i} \cdot a_i)G_1 \\
(\sum_{i=1}^m L_{2,i} \cdot a_i) G_1 \\
(\sum_{i=1}^m L_{3,i} \cdot a_i) G_1 \\
(\sum_{i=1}^m L_{4,i} \cdot a_i) G_1 \\
(\sum_{i=1}^m L_{5,i} \cdot a_i) G_1 \\
\end{array} \right]
```

```math
RaG_2 = \begin{bmatrix}
0 & 0 & 1 & 0 & 0 & 0 & 0 & 0 \\
0 & 0 & 1 & 0 & 0 & 0 & 0 & 0 \\
0 & 0 & 0 & 1 & 0 & 0 & 0 & 0 \\
0 & 0 & 0 & 0 & 0 & 0 & 1 & 0 \\
0 & 0 & 0 & 1 & 0 & 0 & 0 & 0 \\
\end{bmatrix} \cdot
\begin{bmatrix}
1G_1 \\
zG_1 \\
xG_1 \\
yG_1 \\
v_1G_1 \\
v_2G_1 \\
v_3G_1 \\
v_4G_1 \\
\end{bmatrix}
=
\left[ \begin{array}{c}
(\sum_{i=1}^m R_{1,i} \cdot a_i)G_2 \\
(\sum_{i=1}^m R_{2,i} \cdot a_i) G_2 \\
(\sum_{i=1}^m R_{3,i} \cdot a_i) G_2 \\
(\sum_{i=1}^m R_{4,i} \cdot a_i) G_2 \\
(\sum_{i=1}^m R_{5,i} \cdot a_i) G_2 \\
\end{array} \right]
```

```math
OaG_1 = \begin{bmatrix}
0 & 0 & 0 & 0 & 1 & 0 & 0 & 0 \\
0 & 0 & 0 & 0 & 0 & 1 & 0 & 0 \\
0 & 0 & 0 & 0 & 0 & 0 & 1 & 0 \\
0 & 0 & 0 & 0 & 0 & 0 & 0 & 1 \\
5 & -1 & 0 & 0 & 0 & 1 & 0 & 1 \\
\end{bmatrix} \cdot
\begin{bmatrix}
1G_1 \\
zG_1 \\
xG_1 \\
yG_1 \\
v_1G_1 \\
v_2G_1 \\
v_3G_1 \\
v_4G_1 \\
\end{bmatrix}
=
\left[ \begin{array}{c}
(\sum_{i=1}^m O_{1,i} \cdot a_i)G_1 \\
(\sum_{i=1}^m O_{2,i} \cdot a_i) G_1 \\
(\sum_{i=1}^m O_{3,i} \cdot a_i) G_1 \\
(\sum_{i=1}^m O_{4,i} \cdot a_i) G_1 \\
(\sum_{i=1}^m O_{5,i} \cdot a_i) G_1 \\
\end{array} \right]
```

## Verification
$LaG_1$, $RaG_2$, and $OaG_1$ are now vectors of elliptic curve points, specifically:

```math
LaG_1, \ OaG_1 \in G_1 \quad \text{and} \quad RaG_2 \in G_2
```

Since all linear combinations (scalars) are now lifted to the elliptic curve, we can use bilinear pairings to compare them constraint-by-constraint.

```math
\left[ \begin{array}{c}
(\sum_{i=1}^m L_{1,i} \cdot a_i)G_1 \\
(\sum_{i=1}^m L_{2,i} \cdot a_i) G_1 \\
(\sum_{i=1}^m L_{3,i} \cdot a_i) G_1 \\
(\sum_{i=1}^m L_{4,i} \cdot a_i) G_1 \\
(\sum_{i=1}^m L_{5,i} \cdot a_i) G_1 \\
\end{array} \right]
\begin{matrix}
\bullet \\
\bullet \\
\bullet \\
\bullet \\
\bullet \\
\end{matrix}
\left[ \begin{array}{c}
(\sum_{i=1}^m R_{1,i} \cdot a_i)G_2 \\
(\sum_{i=1}^m R_{2,i} \cdot a_i) G_2 \\
(\sum_{i=1}^m R_{3,i} \cdot a_i) G_2 \\
(\sum_{i=1}^m R_{4,i} \cdot a_i) G_2 \\
(\sum_{i=1}^m R_{5,i} \cdot a_i) G_2 \\
\end{array} \right]

\stackrel{?}{=}

\left[ \begin{array}{c}
(\sum_{i=1}^m O_{1,i} \cdot a_i)G_1 \\
(\sum_{i=1}^m O_{2,i} \cdot a_i) G_1 \\
(\sum_{i=1}^m O_{3,i} \cdot a_i) G_1 \\
(\sum_{i=1}^m O_{4,i} \cdot a_i) G_1 \\
(\sum_{i=1}^m O_{5,i} \cdot a_i) G_1 \\
\end{array} \right]
\begin{matrix}
\bullet \\
\bullet \\
\bullet \\
\bullet \\
\bullet \\
\end{matrix}
\left[ \begin{array}{c}
G_2 \\
G_2 \\
G_2 \\
G_2 \\
G_2 \\
\end{array} \right]
```

That is for every row $k$, we take the pairing of each $G1$ and $G2$ point on both sides of the equality and compare them:

```math
e \left((\sum_{i=1}^m L_{k,i} \cdot a_i)G_1, \ (\sum_{i=1}^m R_{k,i} \cdot a_i)G_2 \right) \ 
\stackrel{?}{=} \ 
e \left((\sum_{i=1}^m O_{k,i} \cdot a_i)G_1, \ G_2 \right)
```

Each pairing on both sides of the equality evaluates to an element in the target group $G_{1 \ 2}$. And the two vectors of $G_{1 \ 2}$ elements will be equal element-wise if and only if the witness satisfies all R1CS constraints. That is:

```math
(L_k \cdot a) \cdot (R_k \cdot a) = (O_k \cdot a) \quad \text{for every row} \ k
```

## References
[RareSkills Book of ZK](https://www.rareskills.io/zk-book), where all the good stuff is. 