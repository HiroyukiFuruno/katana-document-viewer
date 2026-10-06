import os
import subprocess
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[3]


class ExternalCargoTests(unittest.TestCase):
    def dry_run(self, recipe, cargo=None):
        environment = os.environ.copy()
        environment.pop("CARGO", None)
        if cargo is not None:
            environment["CARGO"] = cargo
        return subprocess.run(["just", "--dry-run", recipe], cwd=ROOT,
                              env=environment, capture_output=True, text=True, encoding="utf-8", check=True).stderr

    def test_external_recipes_keep_cargo_reachable_after_cd(self):
        recipes = ("storybook-coordinate-contract-check-core", "storybook-hover-contract-check-core",
                   "storybook-task-checkbox-check-core")
        for recipe in recipes:
            output = self.dry_run(recipe)
            self.assertIn('cargo test -p katana-ui-core', output)
            self.assertNotIn('cargo-guard', output)
        self.assertIn('scripts/maintenance/cargo-guard clippy', self.dry_run("lint"))

    def test_explicit_cargo_override_is_retained_for_both_scopes(self):
        for recipe in ("storybook-coordinate-contract-check-core", "lint"):
            output = self.dry_run(recipe, "chosen-cargo")
            self.assertIn('chosen-cargo ', output)
            self.assertNotIn('cargo-guard', output)


if __name__ == "__main__":
    unittest.main()
