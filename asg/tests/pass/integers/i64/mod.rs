// Copyright (C) 2019-2020 Aleo Systems Inc.
// This file is part of the Leo library.

// The Leo library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The Leo library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the Leo library. If not, see <https://www.gnu.org/licenses/>.

// Copyright (C) 2019-2020 Aleo Systems Inc.
// This file is part of the Leo library.

// The Leo library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The Leo library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the Leo library. If not, see <https://www.gnu.org/licenses/>.

use super::IntegerTester;

test_int!(TestI64);

#[test]
fn test_i64_min() {
    TestI64::test_min();
}

#[test]
fn test_i64_max() {
    TestI64::test_max();
}

#[test]
fn test_i64_neg() {
    TestI64::test_negate();
}

#[test]
fn test_i64_neg_zero() {
    TestI64::test_negate_zero();
}

#[test]
fn test_i64_add() {
    TestI64::test_add();
}

#[test]
fn test_i64_sub() {
    TestI64::test_sub();
}

#[test]
fn test_i64_mul() {
    TestI64::test_mul();
}

#[test]
#[ignore] // takes 2 minutes
fn test_i64_div() {
    TestI64::test_div();
}

#[test]
fn test_i64_pow() {
    TestI64::test_pow();
}

#[test]
fn test_i64_eq() {
    TestI64::test_eq();
}

#[test]
fn test_i64_ne() {
    TestI64::test_ne();
}

#[test]
fn test_i64_ge() {
    TestI64::test_ge();
}

#[test]
fn test_i64_gt() {
    TestI64::test_gt();
}

#[test]
fn test_i64_le() {
    TestI64::test_le();
}

#[test]
fn test_i64_lt() {
    TestI64::test_lt();
}

#[test]
fn test_i64_console_assert() {
    TestI64::test_console_assert();
}

#[test]
fn test_i64_ternary() {
    TestI64::test_ternary();
}
