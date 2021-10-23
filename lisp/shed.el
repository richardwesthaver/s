;;; shed.el --- Shed Emacs Package -*- lexical-binding: t; -*-
;; Copyright (C) 2021  ellis
;; 
;; Author: ellis
;; Keywords: local
;; 
;; This program is free software; you can redistribute it and/or modify
;; it under the terms of the GNU General Public License as published by
;; the Free Software Foundation, either version 3 of the License, or
;; (at your option) any later version.
;; 
;; This program is distributed in the hope that it will be useful,
;; but WITHOUT ANY WARRANTY; without even the implied warranty of
;; MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
;; GNU General Public License for more details.
;; 
;; You should have received a copy of the GNU General Public License
;; along with this program.  If not, see <https://www.gnu.org/licenses/>.
;; 
;; Commentary:
;; 
;; This package provides functions for interacting with the local Shed
;; development system.
;; 
;;; Code:
(defgroup shed nil
  "Shed Emacs Modules")

(defcustom shed-dir "~/shed/"
  "Shed directory.")

(defcustom shed-data-dir "~/shed/data/"
  "Shed data directory.")

(setq bookmark-default-file "~/shed/data/emacs/bookmarks")

;;;; Daemon 
(defgroup shed-daemon ()
  "Shed emacs daemon settings"
  :group 'shed)

(defcustom server-after-make-frame-hook nil
  "Hook run when the Shed server creates a client frame.
The created frame is selected when the hook is called."
  :type 'hook
  :version "27.1")

(defcustom server-done-hook nil
  "Hook run when done editing a buffer for the Shed server."
  :type 'hook)

(defvar server-process nil
  "The current server process.")

(defvar server-clients nil
  "List of current server clients.
Each element is a process.")

(defun server-shutdown ()
  "Save buffers, Quit, and Shutdown (kill) server"
  (interactive)
  (save-some-buffers)
  (kill-emacs))

(defun signal-restart-server ()
  "Handler for SIGUSR1 signal, to (re)start an emacs server.

Can be tested from within emacs with:
  (signal-process (emacs-pid) 'sigusr1)

or from the command line with:
$ kill -USR1 <emacs-pid>
$ emacsclient -c
"
  (interactive)
  (server-force-delete)
  (server-start)
  )

(define-key special-event-map [sigusr1] 'signal-restart-server)

;;;; Process
(defun msg-me (process event)
  (princ
   (format "Process: %s had the event `%s'" process event)))

(defun ordinary-insertion-filter (proc string)
  (when (buffer-live-p (process-buffer proc))
    (with-current-buffer (process-buffer proc)
      (let ((moving (= (point) (process-mark proc))))

        (save-excursion
          ;; Insert the text, advancing the process marker.
          (goto-char (process-mark proc))
          (insert string)
          (set-marker (process-mark proc) (point)))
        (if moving (goto-char (process-mark proc)))))))

;;;; Network
;;;###autoload
(defun net-check-opts ()
  ;; https://gnu.huihoo.org/emacs/24.4/emacs-lisp/Network-Options.html#Network-Options
  ;; non-blocking
  (featurep 'make-network-process '(:nowait t))
  ;; UNIX socket
  ;(featurep 'make-network-process '(:family local))
  ;; UDP
  (featurep 'make-network-process '(:type datagram)))

(defvar shed-cmd-server-port 62824
  "port of the shed-status broadcaster")

(defvar shed-cmd-server-clients '() 
  "alist where KEY is a client process and VALUE is the string")

(defun shed-cmd-make-client (host port)
  (make-network-process
   :name "shed-cmd-client"
   :coding 'binary
   :host host
   :service port
   :type 'datagram))

(defun shed-cmd-server-start nil
  "starts a shed-cmd broadcaster over udp"
  (interactive)
  (unless (process-status "shed-cmd-server")
    (make-network-process :name "shed-cmd-server" :buffer "*shed-cmd-server*" :family 'ipv4 :service shed-cmd-server-port :type 'datagram :coding 'binary :sentinel 'shed-cmd-server-sentinel :filter 'shed-cmd-server-filter :server 't) 
    (setq shed-cmd-server-clients '())
    )
  )

(defun shed-cmd-server-stop nil
  "stop a shed-cmd-server"
  (interactive)
  (while  shed-cmd-server-clients
    (delete-process (car (car shed-cmd-server-clients)))
    (setq shed-cmd-server-clients (cdr shed-cmd-server-clients)))
  (delete-process "shed-cmd-server")
  )

(defun shed-cmd-server-filter (proc string)   
  (let ((pending (assoc proc shed-cmd-server-clients))
        message
        index)
    ;;create entry if required
    (unless pending
      (setq shed-cmd-server-clients (cons (cons proc "") shed-cmd-server-clients))
      (setq pending  (assoc proc shed-cmd-server-clients)))
    (setq message (concat (cdr pending) string))
    (while (setq index (string-match "\n" message))
      (setq index (1+ index))
      (process-send-string proc (substring message 0 index))
      (shed-cmd-server-log  (substring message 0 index) proc)
      (setq message (substring message index)))
    (setcdr pending message))
  )

(defun shed-cmd-server-sentinel (proc msg)
  (when (string= msg "connection broken by remote peer\n")
    (setq echo-server-clients (assq-delete-all proc echo-server-clients))
    (echo-server-log (format "client %s has quit" proc))))

;;from server.el
;;;###autoload
(defun shed-cmd-server-log (string &optional client)
  "If a *shed-cmd-server* buffer exists, write STRING to it for logging purposes."
  (if (get-buffer "*shed-cmd-server*")
      (with-current-buffer "*shed-cmd-server*"
        (goto-char (point-max))
        (insert (current-time-string)
                (if client (format " %s:" client) " ")
                string)
        (or (bolp) (newline)))))


;;;; Coding 
(defun shed-proto-insert-string (string)
  (insert string 0 (make-string (- 3 (% (length string) 4)) 0)))

(defun shed-proto-insert-float32 (value)
  (let (s (e 0) f)
    (cond
     ((string= (format "%f" value) (format "%f" -0.0))
      (setq s 1 f 0))
     ((string= (format "%f" value) (format "%f" 0.0))
      (setq s 0 f 0))
     ((= value 1.0e+INF)
      (setq s 0 e 255 f (1- (expt 2 23))))
     ((= value -1.0e+INF)
      (setq s 1 e 255 f (1- (expt 2 23))))
     ((string= (format "%f" value) (format "%f" 0.0e+NaN))
      (setq s 0 e 255 f 1))
     (t
      (setq s (if (>= value 0.0)
		  (progn (setq f value) 0)
		(setq f (* -1 value)) 1))
      (while (>= (* f (expt 2.0 e)) 2.0) (setq e (1- e)))
      (if (= e 0) (while (< (* f (expt 2.0 e)) 1.0) (setq e (1+ e))))
      (setq f (round (* (1- (* f (expt 2.0 e))) (expt 2 23)))
	    e (+ (* -1 e) 127))))
    (insert (+ (lsh s 7) (lsh (logand e #XFE) -1))
	    (+ (lsh (logand e #X01) 7) (lsh (logand f #X7F0000) -16))
	    (lsh (logand f #XFF00) -8)
	    (logand f #XFF))))

(defun shed-proto-insert-int32 (value)
  (let (bytes)
    (dotimes (i 4)
      (push (% value 256) bytes)
      (setq value (/ value 256)))
    (dolist (byte bytes)
      (insert byte))))

(setq header-spec
      '((dest-ip   ip)
        (src-ip    ip)
        (dest-port u16)
        (src-port  u16)))
(setq data-spec
      '((type      u8)
        (opcode    u8)
        (length    u16)  ; network byte order
        (id        strz 8)
        (data      vec (length))
        (align     4)))
(setq packet-spec
      '((header    struct header-spec)
        (counters  vec 2 u32r)   ; little endian order
        (items     u8)
        (fill      3)
        (item      repeat (items)
                   (struct data-spec))))

;;;; pkg 
(provide 'shed)
;;; shed.el ends here