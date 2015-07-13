import org.scalatest._

import scala.runtime.RichInt

class ChopSpec extends FunSpec {
  describe("A Chopper") {
    it("chops empty arrays") {
      assert(-1 == Chop.chop(0, List()))
    }
  }

}
