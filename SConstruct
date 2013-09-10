# SConstruct

import os

debug = True

env = Environment(tools=['mingw'])

if debug:
  env.Append(CCFLAGS = ['-O0', '-g', '-DDEBUG', '-D_DEBUG'])
else:
  env.Append(CCFLAGS = ['-O3', '-march=native', '-DNDEBUG', '-D_NDEBUG'])

env.Append(CCFLAGS = ['-pipe', '-Wall', '-Wextra', '-pedantic-errors', '-std=c++1y'])

env.Append(CCFLAGS = ['-DUNICODE', '-D_UNICODE'])
env.Append(LINKFLAGS = ['-mwindows'])

boost_root = os.environ['BOOST_DIR']
env.Append(CPPPATH = [os.path.join(boost_root, 'include')])
env.Append(LIBPATH = [os.path.join(boost_root, 'lib')])


app = env.Program('Random3dMaze', [
  'src/main.cpp',
  env.RES('res/main.rc'),
])
Default(app)


test_env = env.Clone()
test_env.Append(LIBS = ['boost_unit_test_framework'])
test = test_env.Program('test', [
  'test/main.cpp',
])
test_env.Alias('check', test, test[0].abspath)
Clean(all, test)
