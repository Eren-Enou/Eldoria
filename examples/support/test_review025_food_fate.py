"""Finite-food attribution controls, not new simulation experiments."""
import unittest

from review025_food_fate import FoodFates


class AttributionTests(unittest.TestCase):
    def test_mixed_meal_is_ambiguous_until_both_units_are_spent(self):
        f = FoodFates([1, 0, 0, 0])
        f.prepared(0, 'prepared0')
        f.debit(0)
        self.assertEqual(f.summary()['consumed'], [0, 1])
        self.assertEqual(f.summary()['units'][0]['status'], 'ambiguous')
        f.debit(0)
        self.assertEqual(f.summary()['consumed'], [1, 1])
        self.assertEqual(f.summary()['units'][0]['status'], 'guaranteed_used')

    def test_joint_count_does_not_sum_individual_uncertainty(self):
        f = FoodFates([0, 0, 0, 0])
        f.prepared(0, 'prepared0')
        f.prepared(0, 'prepared1')
        f.debit(0)
        self.assertEqual(f.summary()['consumed'], [1, 1])
        self.assertEqual([u['status'] for u in f.summary()['units']], ['ambiguous'] * 2)

    def test_real_transfer_does_not_identify_later_mixed_meal(self):
        f = FoodFates([0, 2, 0, 0])
        f.prepared(0, 'prepared0')
        f.debit(0, 1)
        f.debit(1)
        self.assertEqual(f.summary()['ever_transferred'], [1, 1])
        self.assertEqual(f.summary()['consumed'], [0, 1])
        self.assertEqual(f.summary()['ever_used'], [1, 1])
        f.debit(1)
        f.debit(1)
        self.assertEqual(f.summary()['consumed'], [1, 1])

    def test_mixed_transfer_keeps_both_possible_owners(self):
        f = FoodFates([1, 0, 0, 0])
        f.prepared(0, 'prepared0')
        f.debit(0, 1)
        self.assertEqual(f.summary()['ever_transferred'], [0, 1])
        self.assertEqual(f.summary()['terminal_assignments'], 2)

    def test_uncirculated_output_is_not_already_nutrition(self):
        f = FoodFates([0, 0, 0, 0])
        f.prepared(2, 'prepared0')
        self.assertEqual(f.summary()['ever_used'], [0, 0])
        self.assertEqual(f.summary()['consumed'], [0, 0])
        self.assertEqual(f.summary()['units'][0]['status'], 'guaranteed_unused')

    def test_no_debit_without_ownership_and_no_self_transfer(self):
        f = FoodFates([0, 0, 0, 0])
        with self.assertRaises(AssertionError):
            f.debit(0)
        f.untagged(0, 1)
        with self.assertRaises(AssertionError):
            f.debit(0, 0)


if __name__ == '__main__':
    unittest.main()
