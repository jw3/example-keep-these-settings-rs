import os
from setuptools import setup, find_namespace_packages
from setuptools_rust import RustExtension


#
# rm -rf fapolicy_analyzer.egg-info/ build/ dist/ && python setup.py bdist_wheel
#
setup(
    name="dywtkts",
    version=os.getenv("VERSION", "snapshot"),
    packages=find_namespace_packages(
        include=["mygui"]
    ),
    setup_requires=["setuptools", "setuptools_rust"],
    zip_safe=False,
    rust_extensions=[RustExtension("dywtkts.rust")],
    include_package_data=True,
    package_data={
        "mygui.glade": ["*.glade"],
    },
)
