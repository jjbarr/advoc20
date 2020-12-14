(import (chicken io) (chicken sort) (srfi 1) (amb) (amb-extras))
(define input
  (with-input-from-file "1in"
    (lambda ()
      (sort (read-list) <))))

(define (sum-2020 n l)
  (cond ((or (null? l) (> (+ n (car l)) 2020)) #f)
        ((= (+ n (car l)) 2020) (car l))
        (else (sum-2020 n (cdr l)))))

(let* ((n1 (amb1 input))
       (n2 (amb1 (delete n1 input))))
  (amb-assert (= (+ n1 n2) 2020))
  (print (* n1 n2)))

(let* ((n1 (amb1 input))
       (n2 (amb1 (delete n1 input)))
       (n3 (amb1 (delete n1 (delete n2 input)))))
  (amb-assert (= (+ n1 n2 n3) 2020))
  (print (* n1 n2 n3)))
