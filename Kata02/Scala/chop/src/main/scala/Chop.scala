object Chop {
  def main(args: Array[String]) {
    println("Chop!")
  }

  def chop[A <% Ordered[A]](item : A, seq : Seq[A]) : Int = {
    if(seq.nonEmpty) {
      val midIndex = seq.length / 2
      if(seq.apply(midIndex) > item) {
        chop(item, seq.slice(0, midIndex / 2))
      } else if(seq.apply(midIndex) < item) {
        chop(item, seq.slice(midIndex + 1, seq.length))
      } else {
        midIndex
      }
    } else {
      -1
    }
  }
}

