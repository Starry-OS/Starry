# The directory of the main repo
WORK_DIR=$1

# The package which needs to be patched
# It always be the package which triggers the test
PATCH_PACKAGE=$2

# The URL of the patch points to
PATCH_TARGET_URL=$3

# The commit hash of the patch
PATCH_COMMIT_HASH=$4

# To run the main repo actions
mkdir .github/workflows/actions
mv $WORK_DIR/.github/workflows/actions/* .github/workflows/actions

# To do the patch for current commit
cargo install dependencies-patch
# cargo run -- ../../ $PATCH_PACKAGE $PATCH_TARGET_URL $PATCH_COMMIT_HASH
dependencies-patch -c $WORK_DIR -n $PATCH_PACKAGE --git-repo $PATCH_TARGET_URL --commit $PATCH_COMMIT_HASH