import org.scalatest._

import scala.runtime.RichInt

class ChopSpec extends FunSpec {
  describe("A Chopper") {
    it("chops empty arrays") {
      assert(-1 == Chop.chop(0, List()))
    }

    it("chops arrays of one element") {
    	assert(-1 == Chop.chop(3, List(1)))
    	assert(0 == Chop.chop(1, List(1)))
    }
  }

}
