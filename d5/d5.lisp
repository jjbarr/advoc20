#!/usr/bin/sbcl --script
(defparameter *input*
  (with-open-file (in "5in")
    (sort
     (loop for line = (read-line in nil)
           while line
           collect (reduce
                    (lambda (a n)
                      (+ (ash a 1) (case n
                                     ((#\F #\L) 0)
                                     ((#\B #\R) 1)
                                     (otherwise 0))))
                    line :initial-value 0))
     #'<)))

(format t "~A~%" (apply #'max *input*))

(defun find-missing (l)
  (if (not (= (cadr l) (1+ (car l))))
      (1+ (car l))
      (find-missing (cdr l))))

(format t "~A~%" (find-missing *input*))
