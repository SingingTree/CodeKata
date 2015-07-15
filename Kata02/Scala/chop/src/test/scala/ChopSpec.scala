import org.scalatest._

class ChopSpec extends FunSpec {
  describe("A Chopper") {
    it("chops an empty array") {
      assert(-1 == Chop.chop(0, List()))
    }

    it("chops a single element array") {
    	assert(-1 == Chop.chop(3, List(1)))
    	assert(0 == Chop.chop(1, List(1)))
    }

    it("chops a three element array") {
      assert(0 == Chop.chop(1, List(1, 3, 5)))
      assert(1 == Chop.chop(3, List(1, 3, 5)))
      assert(2 == Chop.chop(5, List(1, 3, 5)))
      assert(-1 == Chop.chop(0, List(1, 3, 5)))
      assert(-1 == Chop.chop(2, List(1, 3, 5)))
      assert(-1 == Chop.chop(4, List(1, 3, 5)))
      assert(-1 == Chop.chop(6, List(1, 3, 5)))
    }

    it("chops a four element array") {
      assert(0 == Chop.chop(1, List(1, 3, 5, 7)))
      assert(1 == Chop.chop(3, List(1, 3, 5, 7)))
      assert(2 == Chop.chop(5, List(1, 3, 5, 7)))
      assert(3 == Chop.chop(7, List(1, 3, 5, 7)))
      assert(-1 == Chop.chop(0, List(1, 3, 5, 7)))
      assert(-1 == Chop.chop(2, List(1, 3, 5, 7)))
      assert(-1 == Chop.chop(4, List(1, 3, 5, 7)))
      assert(-1 == Chop.chop(6, List(1, 3, 5, 7)))
      assert(-1 == Chop.chop(8, List(1, 3, 5, 7)))
    }
  }

}
