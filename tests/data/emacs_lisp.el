;; 21 lines 11 code 6 comments 4 blanks

                                        ; This is a comment line
;; This too!
;;; This 3!
;;;; This 4!

(setq some-global-var nil)              ;Comment

;;;###autoload
(defun some-fn ()
  "Some function."
  (interactive)
  (message "I am some function"))

(defun fundamental-mode ()
  "Major mode not specialized for anything in particular.
Other major modes are defined by comparison with this one."
  (interactive)
  (kill-all-local-variables)
  (run-mode-hooks))
