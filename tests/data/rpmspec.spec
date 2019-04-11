# 42 lines 22 code 4 comments 16 blanks

Name:           example
Version:        0.0.1
Release:        1%{?dist}
Summary:        an example specfile

Group:          
URL:            
Source0:        

# test comments for requirements
BuildRequires:  
Requires:       

%description


%prep
%setup -q


# build the project
%build
%configure
make build


# install the files here
%install
make install


%clean


%files
%defattr(-,root,root,-)
%doc


%changelog
