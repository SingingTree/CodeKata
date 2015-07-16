object Chop {
  def main(args: Array[String]) {
    println("Chop!")
  }

  def chop[T](item : T, seq : Seq[T], indexOffset : Int = 0)(implicit f : T => Ordered[T]) : Int = {
    if(seq.nonEmpty) {
      val midIndex = seq.length / 2
      if(seq.apply(midIndex) > item) {
        chop(item, seq.slice(0, midIndex), indexOffset)
      } else if(seq.apply(midIndex) < item) {
        chop(item, seq.slice(midIndex + 1, seq.length), indexOffset + midIndex + 1)
      } else {
        indexOffset + midIndex
      }
    } else {
      -1
    }
  }
}

