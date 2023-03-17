===========
``regress``
===========

|PyPI| |Pythons| |CI|

.. |PyPI| image:: https://img.shields.io/pypi/v/regress.svg
  :alt: PyPI version
  :target: https://pypi.org/project/regress/

.. |Pythons| image:: https://img.shields.io/pypi/pyversions/regress.svg
  :alt: Supported Python versions
  :target: https://pypi.org/project/regress/

.. |CI| image:: https://github.com/Julian/regress/workflows/CI/badge.svg
  :alt: Build status
  :target: https://github.com/Julian/regress/actions?query=workflow%3ACI


Python bindings to the Rust `regress <https://docs.rs/regress/latest/regress/>`_ crate, exposing ECMA regular expressions.


.. code:: python

    >>> from regress import Regex
    >>> regex = Regex(r"\d{4}")
    >>> regex.find("2020-20-05") is not None
    True
