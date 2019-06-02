FROM microsoft/dotnet:2.2.1-aspnetcore-runtime

RUN apt update && apt install apt-utils apt-transport-https dirmngr wget curl gnupg software-properties-common -y --no-install-recommends

## Workaround for apt/jre issues
RUN mkdir -p /usr/share/man/man1
RUN apt install apt-utils -y --no-install-recommends

## Install JRE
RUN apt install default-jre -y --no-install-recommends

# Powershell
RUN curl https://packages.microsoft.com/keys/microsoft.asc | apt-key add -
RUN sh -c 'echo "deb [arch=amd64] https://packages.microsoft.com/repos/microsoft-debian-stretch-prod stretch main" > /etc/apt/sources.list.d/microsoft.list'
RUN apt update && apt install powershell -y --no-install-recommends

# Git: install from sources to get latest
RUN apt install make libssl-dev libghc-zlib-dev libcurl4-gnutls-dev libexpat1-dev libpcre2-dev gettext unzip -y --no-install-recommends
RUN export USE_LIBPCRE=yes && \
    cd /usr/src/ && \
    wget https://github.com/git/git/archive/v2.20.1.tar.gz -O git.tar.gz && \
    tar -xf git.tar.gz && \
    cd git-* && \
    make prefix=/usr/local all && \
    make prefix=/usr/local install

# Git LFS
RUN curl -s https://packagecloud.io/install/repositories/github/git-lfs/script.deb.sh | bash
RUN apt install -y git-lfs

# Git settings
RUN git config --global diff.renameLimit 999999
RUN git config --global user.email 'eng.codeanalysis@trilogy.com'
RUN git config --global user.name 'caflow'

# Git-SVN dependencies
RUN apt install -y libsvn-perl
RUN export PERL_MM_USE_DEFAULT=1 && cpan Term::ReadKey

# Cloc
RUN wget -O /usr/bin/cloc https://github.com/AlDanial/cloc/releases/download/1.80/cloc-1.80.pl && chmod +x /usr/bin/cloc

# Python
# cvs2git does not work with Python 3.x.
RUN apt install python2.7 python-pip -y --no-install-recommends

# CVS
RUN apt install cvs -y

# cvs2git
RUN cd /usr/src/ && \
    wget http://cvs2svn.tigris.org/files/documents/1462/49543/cvs2svn-2.5.0.tar.gz -O cvs2svn.tar.gz && \
    tar -xzf cvs2svn.tar.gz && \
    cd cvs2svn-2.5.0/ && \
    make install

# Ruby & Linguist
RUN apt install ruby-full build-essential libicu-dev cmake pkg-config -y --no-install-recommends
RUN gem install github-linguist --no-ri --no-rdoc
RUN gem install bugspots

# Tokei
RUN wget https://github.com/XAMPPRocky/tokei/releases/download/v9.1.1/tokei-v9.1.1-x86_64-unknown-linux-gnu.tar.gz
RUN tar -xvzf tokei-v9.1.1-x86_64-unknown-linux-gnu.tar.gz && mv tokei /usr/bin
