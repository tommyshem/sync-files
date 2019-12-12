package sync

import "testing"

// TestSum tests the string for correct hash generated
func TestHashString(t *testing.T) {
	stringToHash := "Hello World! 1234567**"
	got := HashString(stringToHash)
	if got != "3c8723969d931f4a1ea70427f2ec7ceb1e921dab3bf5ec72b68ff84197a6551500765095496630ace00908707b6c31df6bb55e9d3b80afee992c221dbca62342" {
		t.Errorf("HashString = %s", got)
	}
}
