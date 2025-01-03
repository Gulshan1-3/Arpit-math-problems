# README: Mathematical Concepts in Gaussian Elimination

This program checks whether a given matrix can be "unlocked" (i.e., whether the system of linear equations represented by the matrix has full rank). It performs **Gaussian elimination** to compute the rank of the matrix and compares it with the number of columns to determine if the system has a unique solution.

## Key Concepts

### 1. **Matrix Representation of a System of Linear Equations**

A matrix can represent a system of linear equations where each row corresponds to an equation and each column corresponds to a variable. For example, the system:

\[
\begin{aligned}
x + 0y + 0z &= 1 \\
0x + y + 0z &= 1 \\
0x + 0y + z &= 1
\end{aligned}
\]

is represented by the following matrix:

\[
A = \begin{bmatrix}
1 & 0 & 0 \\
0 & 1 & 0 \\
0 & 0 & 1
\end{bmatrix}
\]

### 2. **Gaussian Elimination**

Gaussian elimination is a method for solving systems of linear equations. It involves performing row operations to transform a given matrix into its **row echelon form** (REF) or **reduced row echelon form** (RREF). The steps involved include:

- **Pivoting**: Selecting a pivot element (a non-zero element) in each column to use as the divisor for eliminating entries below it.
- **Row Swapping**: Swapping rows to place a non-zero pivot in the leading entry of each row.
- **Row Scaling**: Dividing a row by its pivot element to make the pivot element equal to 1.
- **Row Elimination**: Subtracting multiples of the pivot row from other rows to eliminate entries below the pivot.

The goal of Gaussian elimination is to transform the matrix into an upper triangular form, from which the solution to the system of equations can be easily found.

### 3. **Rank of a Matrix**

The rank of a matrix is the maximum number of linearly independent rows (or columns). A matrix has full rank if its rank is equal to the smaller of the number of rows or columns. For an \( m \times n \) matrix, the rank is at most \( \min(m, n) \).

In this program, the rank is determined by the number of non-zero rows in the row echelon form of the matrix. If the rank of the matrix equals the number of columns, the matrix has full rank, which implies that the system has a unique solution.

### 4. **Tolerance for Floating-Point Comparisons**

Due to the limitations of representing real numbers in a computer, floating-point comparisons are prone to small rounding errors. To address this issue, a **tolerance** value is used to determine whether a matrix element is considered "non-zero." If an element's absolute value is greater than the tolerance, it is considered non-zero; otherwise, it is treated as zero.

In this code, the tolerance is set to \( 1 \times 10^{-9} \), and elements whose absolute value is less than this tolerance are treated as zero.

### 5. **Row Echelon Form (REF)**

A matrix is in row echelon form if it satisfies the following conditions:

1. All rows containing only zeros are at the bottom.
2. The leading entry (first non-zero entry) in each row is to the right of the leading entry in the row above it.
3. The leading entry in each non-zero row is 1, and all other entries in the column containing the leading entry are zero.

The goal of Gaussian elimination is to transform the matrix into this form, which allows for easy back substitution to solve the system of equations.

### 6. **Pivoting in Gaussian Elimination**

In Gaussian elimination, a 
