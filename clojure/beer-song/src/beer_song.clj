(ns beer-song)
(use '[clojure.string :only (join)])

(defn- num-bottles [num upper?]
  (cond
    (= num 0) (str (if upper? "N" "n") "o more bottles")
    (= num 1) "1 bottle"
    :else (str num " bottles")))

(defn- beer-on-wall [num upper?]
  (str (num-bottles num upper?) " of beer on the wall"))

(defn- beer [num]
  (str (num-bottles num false) " of beer"))

(defn- final-verse []
  (str "Go to the store and buy some more, "
       (beer-on-wall 99 false) "."))

(defn- mid-verse [num]
  (str "Take " (if (= num 0) "it" "one") " down and pass it around, "
       (beer-on-wall num false) "."))

(defn verse [num]
  (str (beer-on-wall num true) ", " (beer num) ".\n"
       (if (= num 0) 
           (final-verse)
           (mid-verse (dec num)))
       "\n"))

(defn sing
  "Given a start and an optional end, returns all verses in this interval. If
  end is not given, the whole song from start is sung."
  ([start] (sing start 0))
  ([start end]
   (str (verse start) 
        (cond (= start 0) ""
              (= start end) ""
              :else (str "\n" (sing (dec start) end))))))
