Name:    fil
Version: %{version}
Release: 1
Summary: Main tool for fil language
License: GPL-2.0-or-later
Group:   Development/Languages
URL:     https://github.com/Gashmob/fil
BugURL:  https://github.com/gashmob/fil/issues

Source0: bin

%description
%{summary}.

%prep
%setup -q -c -T
cp -r %{SOURCE0}/fil .

%build

%install
mkdir -p %{buildroot}/usr/bin
ls
cp -a fil %{buildroot}/usr/bin

%files
/usr/bin/fil
