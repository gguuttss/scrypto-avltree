use pretty_assertions::assert_eq;
use scrypto::prelude::*;
use scrypto_testenv::TestHelperExecution;

mod helper_avl_tree;

use helper_avl_tree::*;

#[test]
fn test_delete_root() {
    let mut helper = TestHelper::new();
    helper.instantiate_default(false);
    helper.insert(1, 1);
    helper.check_health();
    helper.execute_expect_success(true);
    helper.delete(1);
    helper.check_health();
    helper.execute_expect_success(true);
    let minimum = i32::MIN;
    let maximum = i32::MAX;
    helper.get_range(minimum, maximum);
    let receipt = helper.execute_expect_success(true);
    let output: Vec<Vec<i32>> = receipt.outputs("get_range");
    let output = output[0].clone();
    println!("Output: {:?}", output);
    assert_eq!(output.len(), 0, "Something is still present in the tree");
}

#[test]
fn three_insert_one_delete_3_insert() {
    let mut helper = TestHelper::new();
    helper.instantiate_default(false);
    let mut insert = vec![];
    let mut delete = vec![];
    let mut should_be_in_tree = HashSet::new();
    for i in 0..4 {
        for mut j in 0..3 {
            j = 3 - j;
            print!("insert: --------------------- {}, {}, {} ", (i * 3) + j, i, j);
            helper.insert((i * 3) + j, (i * 3) + j);
            should_be_in_tree.insert((i * 3) + j);
            insert.push((i * 3) + j);
            helper.check_health();
            helper.execute_expect_success(true);
        }
        helper.delete(i * 2 + 1);
        should_be_in_tree.remove(&(i * 2 + 1));
        delete.push(i * 2 + 1);
        helper.check_health();
        helper.execute_expect_success(true);
    }
    println!("insert: {:?}", insert);
    println!("delete: {:?}", delete);
    println!("should_be_in_tree: {:?}", should_be_in_tree);
    let minimum = i32::MIN;
    let maximum = i32::MAX;
    helper.get_range(minimum, maximum);
    let receipt = helper.execute_expect_success(true);
    let output: Vec<Vec<i32>> = receipt.outputs("get_range");
    let output = output[0].clone();
    println!("Output: {:?}", output);
    println!("should_be_in_tree: {:?}", should_be_in_tree);
    for i in should_be_in_tree.iter() {
        assert!(output.contains(&i), "i not contained in the tree {}", i);
    }
    for i in output.iter() {
        assert!(should_be_in_tree.contains(&i), "i not contained in the tree {}", i);
    }
}

#[test]
fn shorten_was_calculated_wrong_because_balance_factor_of_delete_was_wrong() {
    let vector: Vec<i32> = vec![5,3,7,1,4,8, 2];
    let to_delete = vec![5];
    test_range(vector, to_delete);
}

#[test]
fn delete_root_and_check_if_replace_parent_is_given_correct() {
    // If 2 has the wrong bf afterwards the parent was given incorrect
    let vec = vec![6, 2, 7, 3];
    let to_delete = vec![6];
    test_range(vec, to_delete);
}

#[test]
fn replace_jumps_over_his_parent_with_rebalance() {
    let vec = vec![6, 2, 8, 1, 3, 7, 4];
    let to_delete = vec![2];
    test_range(vec, to_delete);
}
#[test]
fn replace_jumps_overhis_parent() {
    let vec = vec![6, 2, 7, 1, 3];
    let to_delete = vec![6];
    test_range(vec, to_delete);
}
#[test]
fn delet_non_existent_and_dont_panic() {
    let vec = vec![6, 2];
    let to_delete = vec![8];
    test_range(vec, to_delete);
}
#[test]
fn deletion_with_2_parents_above_but_only_one_balance() {
    let vector: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];
    let to_delete = vec![0, 1, 2, 3];
    test_range(vector, to_delete);
}
#[test]
fn test_more_than_two_balances_in_delete() {
    let vector = vec![25, 20, 30, 10, 23, 26, 33, 31];
    let to_delete = vec![25];
    test_range(vector, to_delete);
}

#[test]
fn delete_is_bf_0_but_not_shorten() {
    let vector = vec![44, 39, 49, 36, 42, 46, 51, 34, 40, 43, 47, 52, 41];
    let to_delete = vec![36];
    test_range(vector, to_delete);
}

#[test]
fn deletion_with_replace_direct_below() {
    let vector: Vec<i32> = vec![15, 14, 17, 16];
    let to_delete = vec![17];
    test_range(vector, to_delete);
}

#[test]
fn delete_and_balance_at_root() {
    let vector: Vec<i32> = vec![7, 5, 15, 3, 6, 11, 17, 4, 16, 18, 20, 14];
    let to_delete = vec![3];
    test_range(vector, to_delete);
}
