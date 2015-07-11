public class ChopIterative<T> implements Chop<T> {
    public int chop(T item, Comparable<T>[] array) {
        int low = 0;
        int width = array.length;
        while(width > 0) {
            int index = low + width / 2;
            if(array[index].compareTo(item) > 0) {
            } else if(array[index].compareTo(item) < 0) {
                width--;
                low = index + 1;
            } else {
                return index;
            }
            width /= 2;
        }
        return -1;
    }
}
