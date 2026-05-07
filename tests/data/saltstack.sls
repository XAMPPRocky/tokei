# 19 lines 12 code 4 comments 3 blanks

# Install nginx package
nginx:
  pkg.installed:
    - name: nginx

# Ensure nginx service is running
nginx_service:
  service.running:
    - name: nginx
    - enable: True
    - require:
      - pkg: nginx

# Configure nginx
/etc/nginx/nginx.conf:
  file.managed:
    - source: salt://nginx/files/nginx.conf
