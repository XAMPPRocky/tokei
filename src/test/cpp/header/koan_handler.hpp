/*
    Copyright (c) 2012 Torbjörn Klatt <opensource@torbjoern-klatt.de>

    Permission is hereby granted, free of charge, to any person
    obtaining a copy of this software and associated documentation
    files (the "Software"), to deal in the Software without
    restriction, including without limitation the rights to use,
    copy, modify, merge, publish, distribute, sublicense, and/or sell
    copies of the Software, and to permit persons to whom the
    Software is furnished to do so, subject to the following
    conditions:

    The above copyright notice and this permission notice shall be
    included in all copies or substantial portions of the Software.

    THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
    EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES
    OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
    NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT
    HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY,
    WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
    FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
    OTHER DEALINGS IN THE SOFTWARE.
*/


#ifndef KOAN_HANDLER_HPP
#define KOAN_HANDLER_HPP

#include <iostream>
#include "fill_me_in_exception.hpp"
#include "koan.hpp"

using namespace std;

/**
 *
 */
class KoanHandler
{
  private:
    int total_num_koans;    //!
    int total_num_passed;   //!

  public:
    /**
     *
     */
    KoanHandler();

    /**
     *
     */
    void eval_koan( Koan obj, void ( Koan::*koan )() );

    /**
     *
     */
    void register_koans( int num_koans );

    /**
     *
     */
    void start();

    /**
     *
     */
    void end();

    /**
     *
     */
    void episode_start( string order );

    /**
     *
     */
    void episode_done( string order );

  private:
    /**
     *
     */
    void print_failure( FillMeInException ex );
};

#endif // KOAN_HANDLER_HPP

// EOF
